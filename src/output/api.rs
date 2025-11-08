use crate::core::models::ScrapedData;
use crate::output::database::PostgresOutput;
use anyhow::Result;
use axum::{
    extract::{Query, State},
    http::{StatusCode, Method, Uri},
    response::{Json, IntoResponse},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

pub type SharedData = Arc<RwLock<Vec<ScrapedData>>>;
pub type SharedDatabase = Option<Arc<PostgresOutput>>;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub query: Option<String>,
    pub source: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    pub format: Option<String>,
    pub source: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub data: SharedData,
    pub database: SharedDatabase,
}

pub struct ApiServer {
    state: AppState,
    port: u16,
}

impl ApiServer {
    pub fn new(data: SharedData, database: Option<Arc<PostgresOutput>>, port: Option<u16>) -> Self {
        Self {
            state: AppState {
                data,
                database,
            },
            port: port.unwrap_or(3000),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let app = self.create_app();
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        log::info!("Starting full-stack server on http://{}", addr);
        log::info!("API endpoints available at http://{}/api/*", addr);
        log::info!("Frontend available at http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }

    fn create_app(&self) -> Router {
        // Configure CORS for development (allow React dev server on 5173)
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
            .allow_headers(Any)
            .allow_credentials(false);

        // Create API routes
        let api_routes = Router::new()
            .route("/api/data", get(get_data))
            .route("/api/search", get(search_data))
            .route("/api/sources", get(get_sources))
            .route("/api/stats", get(get_stats))
            .route("/api/health", get(health_check))
            .route("/api/export/json", get(export_json))
            .route("/api/export/csv", get(export_csv))
            .route("/api/update", post(update_data))
            .route("/api/scrape", post(trigger_scrape))
            .with_state(self.state.clone())
            .layer(cors)
            .layer(TraceLayer::new_for_http());

        // Check if frontend dist folder exists
        let frontend_path = PathBuf::from("frontend/dist");

        if frontend_path.exists() && frontend_path.is_dir() {
            log::info!("Serving static files from frontend/dist");

            // Serve static files with fallback to index.html for SPA routing
            // We need to handle this manually to get proper 200 status codes for SPA routes
            Router::new()
                .merge(api_routes)
                .fallback(handle_frontend)
        } else {
            log::warn!("Frontend dist folder not found at {:?}. Only serving API endpoints.", frontend_path);
            log::warn!("Run 'make build-frontend' or 'cd frontend && npm install && npm run build' to build the frontend.");
            api_routes
        }
    }

    pub async fn update_data(&self, new_data: Vec<ScrapedData>) -> Result<()> {
        use crate::output::database::DatabaseOutput;

        // Update in-memory data
        let mut data = self.state.data.write().await;
        *data = new_data.clone();
        log::info!("API in-memory data updated with {} items", data.len());

        // Update database if available
        if let Some(db) = self.state.database.as_ref() {
            match db.save(&new_data).await {
                Ok(count) => log::info!("Saved {} items to database", count),
                Err(e) => log::warn!("Failed to save to database: {}", e),
            }
        }

        Ok(())
    }

    pub fn get_state(&self) -> &AppState {
        &self.state
    }
}

async fn health_check() -> (StatusCode, Json<HashMap<&'static str, &'static str>>) {
    let mut response = HashMap::new();
    response.insert("status", "healthy");
    response.insert("service", "rust-scraper-pro");
    response.insert("version", "1.0.0");
    
    (StatusCode::OK, Json(response))
}

async fn get_data(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> (StatusCode, Json<Vec<ScrapedData>>) {
    // Try database first if available
    if let Some(db) = state.database.as_ref() {
        match db.get_all(
            params.limit.map(|l| l as i64),
            params.offset.map(|o| o as i64)
        ).await {
            Ok(mut results) => {
                // Apply additional filters
                if let Some(query) = &params.query {
                    results.retain(|item| {
                        item.title.as_ref().map(|t| t.to_lowercase().contains(&query.to_lowercase())).unwrap_or(false) ||
                        item.content.as_ref().map(|c| c.to_lowercase().contains(&query.to_lowercase())).unwrap_or(false)
                    });
                }

                if let Some(source) = &params.source {
                    results.retain(|item| item.source.to_lowercase().contains(&source.to_lowercase()));
                }

                if let Some(category) = &params.category {
                    results.retain(|item|
                        item.category.as_ref()
                            .map(|c| c.to_lowercase().contains(&category.to_lowercase()))
                            .unwrap_or(false)
                    );
                }

                log::info!("Retrieved {} items from database", results.len());
                return (StatusCode::OK, Json(results));
            }
            Err(e) => {
                log::warn!("Database query failed, falling back to in-memory: {}", e);
            }
        }
    }

    // Fallback to in-memory data
    let data_guard = state.data.read().await;
    let mut results: Vec<ScrapedData> = data_guard.clone();

    // Apply filters
    if let Some(query) = &params.query {
        results.retain(|item| {
            item.title.as_ref().map(|t| t.to_lowercase().contains(&query.to_lowercase())).unwrap_or(false) ||
            item.content.as_ref().map(|c| c.to_lowercase().contains(&query.to_lowercase())).unwrap_or(false)
        });
    }

    if let Some(source) = &params.source {
        results.retain(|item| item.source.to_lowercase().contains(&source.to_lowercase()));
    }

    if let Some(category) = &params.category {
        results.retain(|item|
            item.category.as_ref()
                .map(|c| c.to_lowercase().contains(&category.to_lowercase()))
                .unwrap_or(false)
        );
    }

    // Apply pagination
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(50);

    let end_index = std::cmp::min(offset + limit, results.len());
    let paginated_results = if offset < results.len() {
        results[offset..end_index].to_vec()
    } else {
        Vec::new()
    };

    log::info!("Retrieved {} items from in-memory cache", paginated_results.len());
    (StatusCode::OK, Json(paginated_results))
}

async fn search_data(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> (StatusCode, Json<Vec<ScrapedData>>) {
    let data_guard = state.data.read().await;
    
    let query = params.query.unwrap_or_default().to_lowercase();
    let source_filter = params.source.map(|s| s.to_lowercase());
    let category_filter = params.category.map(|c| c.to_lowercase());

    let results: Vec<ScrapedData> = data_guard
        .iter()
        .filter(|item| {
            // Text search in title and content
            let matches_text = query.is_empty() || 
                item.title.as_ref().map(|t| t.to_lowercase().contains(&query)).unwrap_or(false) ||
                item.content.as_ref().map(|c| c.to_lowercase().contains(&query)).unwrap_or(false);
            
            // Source filter
            let matches_source = if let Some(ref source) = source_filter {
                item.source.to_lowercase().contains(source)
            } else {
                true
            };

            // Category filter
            let matches_category = if let Some(ref category) = category_filter {
                item.category.as_ref()
                    .map(|c| c.to_lowercase().contains(category))
                    .unwrap_or(false)
            } else {
                true
            };

            matches_text && matches_source && matches_category
        })
        .cloned()
        .collect();

    (StatusCode::OK, Json(results))
}

async fn get_sources(State(state): State<AppState>) -> (StatusCode, Json<Vec<String>>) {
    // Try database first
    if let Some(db) = state.database.as_ref() {
        if let Ok(sources) = db.get_sources().await {
            log::info!("Retrieved {} sources from database", sources.len());
            return (StatusCode::OK, Json(sources));
        }
    }

    // Fallback to in-memory
    let data_guard = state.data.read().await;
    let mut sources: Vec<String> = data_guard
        .iter()
        .map(|item| item.source.clone())
        .collect();
    
    sources.sort();
    sources.dedup();
    
    (StatusCode::OK, Json(sources))
}

async fn get_stats(State(state): State<AppState>) -> (StatusCode, Json<HashMap<String, usize>>) {
    let data_guard = state.data.read().await;
    let mut stats = HashMap::new();
    
    stats.insert("total_items".to_string(), data_guard.len());
    
    let source_counts: HashMap<String, usize> = data_guard
        .iter()
        .fold(HashMap::new(), |mut acc, item| {
            *acc.entry(item.source.clone()).or_insert(0) += 1;
            acc
        });
    
    stats.insert("unique_sources".to_string(), source_counts.len());
    
    let items_with_content = data_guard.iter().filter(|item| item.content.is_some()).count();
    stats.insert("items_with_content".to_string(), items_with_content);
    
    let items_with_price = data_guard.iter().filter(|item| item.price.is_some()).count();
    stats.insert("items_with_price".to_string(), items_with_price);

    (StatusCode::OK, Json(stats))
}

async fn export_json(State(state): State<AppState>) -> (StatusCode, Json<Vec<ScrapedData>>) {
    let data_guard = state.data.read().await;
    (StatusCode::OK, Json(data_guard.clone()))
}

async fn export_csv(State(state): State<AppState>) -> (StatusCode, String) {
    let data_guard = state.data.read().await;
    
    let mut wtr = csv::Writer::from_writer(Vec::new());
    
    // Write header
    if wtr.write_record(&["id", "source", "url", "title", "content", "price", "author", "timestamp", "category"]).is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "CSV generation error".to_string());
    }

    for item in data_guard.iter() {
        if wtr.write_record(&[
            &item.id,
            &item.source,
            &item.url,
            item.title.as_deref().unwrap_or(""),
            item.content.as_deref().unwrap_or(""),
            &item.price.map(|p| p.to_string()).unwrap_or_default(),
            item.author.as_deref().unwrap_or(""),
            &item.timestamp.to_rfc3339(),
            item.category.as_deref().unwrap_or(""),
        ]).is_err() {
            return (StatusCode::INTERNAL_SERVER_ERROR, "CSV generation error".to_string());
        }
    }

    match wtr.into_inner() {
        Ok(bytes) => {
            let csv_string = String::from_utf8_lossy(&bytes).to_string();
            (StatusCode::OK, csv_string)
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "CSV generation error".to_string()),
    }
}

async fn update_data(
    State(state): State<AppState>,
    Json(new_data): Json<Vec<ScrapedData>>,
) -> (StatusCode, Json<HashMap<&'static str, String>>) {
    let mut data_guard = state.data.write().await;
    let count = new_data.len();
    *data_guard = new_data;

    let mut response = HashMap::new();
    response.insert("status", "success".to_string());
    response.insert("message", "Data updated successfully".to_string());
    response.insert("items_count", count.to_string());

    (StatusCode::OK, Json(response))
}

// Frontend handler - serves static files or index.html for SPA routing
async fn handle_frontend(uri: Uri) -> impl IntoResponse {
    let path = uri.path();

    // API routes should never reach here (they're handled by api_routes)
    if path.starts_with("/api/") {
        return (StatusCode::NOT_FOUND, "API endpoint not found").into_response();
    }

    // Try to serve the file from frontend/dist
    let file_path = format!("frontend/dist{}", path);
    let file_path = std::path::Path::new(&file_path);

    // If path is "/" or empty, serve index.html
    if path == "/" || path.is_empty() {
        return serve_index_html().await;
    }

    // Check if the file exists and is a file (not a directory)
    if file_path.exists() && file_path.is_file() {
        // Serve the file
        match tokio::fs::read(file_path).await {
            Ok(content) => {
                // Determine content type from file extension
                let content_type = get_content_type(path);
                return (
                    StatusCode::OK,
                    [(axum::http::header::CONTENT_TYPE, content_type)],
                    content
                ).into_response();
            }
            Err(_) => return serve_index_html().await,
        }
    }

    // For all other routes (non-existent files), serve index.html for SPA routing
    serve_index_html().await
}

// Helper to serve index.html
async fn serve_index_html() -> axum::response::Response {
    match tokio::fs::read_to_string("frontend/dist/index.html").await {
        Ok(content) => (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
            content
        ).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            "Frontend not found. Build the frontend first."
        ).into_response(),
    }
}

// Handler for triggering a new scrape
async fn trigger_scrape(
    State(state): State<AppState>,
) -> (StatusCode, Json<HashMap<String, serde_json::Value>>) {
    use crate::core::config::Config;
    use crate::core::scraper::ScraperEngine;
    use crate::processors::pipeline::ProcessingPipeline;
    use crate::sources::{EcommerceSource, SourceType};
    use crate::utils::cache::HtmlCache;
    use crate::output::database::DatabaseOutput;
    use std::sync::Arc;

    log::info!("API: Triggering new scrape request");

    // Create a scraper engine for this request
    let config = match Config::load("config/settings.toml").await {
        Ok(cfg) => cfg,
        Err(e) => {
            log::error!("Failed to load config: {}", e);
            let mut response = HashMap::new();
            response.insert("status".to_string(), serde_json::Value::String("error".to_string()));
            response.insert("message".to_string(), serde_json::Value::String(format!("Failed to load config: {}", e)));
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    let cache = Arc::new(HtmlCache::new_html_cache(1000, 3600));
    let pipeline = ProcessingPipeline::new();
    let mut engine = ScraperEngine::new(config, pipeline, Some(cache));

    // Scrape from books.toscrape.com
    let sources = vec![
        SourceType::Ecommerce(
            EcommerceSource::new("https://books.toscrape.com")
                .with_name("Books to Scrape")
        ),
    ];

    let mut all_scraped_data = Vec::new();

    for source in sources {
        match engine.scrape_source(source).await {
            Ok(data) => {
                log::info!("Successfully scraped {} items", data.len());
                all_scraped_data.extend(data);
            }
            Err(e) => {
                log::error!("Failed to scrape: {}", e);
                let mut response = HashMap::new();
                response.insert("status".to_string(), serde_json::Value::String("error".to_string()));
                response.insert("message".to_string(), serde_json::Value::String(format!("Scraping failed: {}", e)));
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
            }
        }
    }

    // Process the data
    let processed_data = match engine.process_data(all_scraped_data).await {
        Ok(data) => data,
        Err(e) => {
            log::error!("Failed to process data: {}", e);
            let mut response = HashMap::new();
            response.insert("status".to_string(), serde_json::Value::String("error".to_string()));
            response.insert("message".to_string(), serde_json::Value::String(format!("Processing failed: {}", e)));
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    let items_count = processed_data.len();

    // Save to database if available
    if let Some(db) = state.database.as_ref() {
        match db.save(&processed_data).await {
            Ok(count) => log::info!("Saved {} items to database", count),
            Err(e) => log::warn!("Failed to save to database: {}", e),
        }
    }

    // Update in-memory data
    {
        let mut data_guard = state.data.write().await;
        *data_guard = processed_data;
        log::info!("Updated in-memory data with {} items", data_guard.len());
    }

    let mut response = HashMap::new();
    response.insert("status".to_string(), serde_json::Value::String("success".to_string()));
    response.insert("message".to_string(), serde_json::Value::String("Scraping completed successfully".to_string()));
    response.insert("items_scraped".to_string(), serde_json::Value::Number(items_count.into()));

    log::info!("Scrape request completed: {} items", items_count);
    (StatusCode::OK, Json(response))
}

// Helper to determine content type from file extension
fn get_content_type(path: &str) -> &'static str {
    if path.ends_with(".js") {
        "text/javascript"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if path.ends_with(".json") {
        "application/json"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".woff") {
        "font/woff"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else {
        "application/octet-stream"
    }
}
