use crate::core::models::ScrapedData;
use anyhow::Result;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SharedData = Arc<RwLock<Vec<ScrapedData>>>;

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

pub struct ApiServer {
    data: SharedData,
    port: u16,
}

impl ApiServer {
    pub fn new(data: SharedData, port: Option<u16>) -> Self {
        Self {
            data,
            port: port.unwrap_or(3000),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let app = self.create_app();
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        log::info!("Starting API server on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }

    fn create_app(&self) -> Router {
        Router::new()
            .route("/api/data", get(get_data))
            .route("/api/search", get(search_data))
            .route("/api/sources", get(get_sources))
            .route("/api/stats", get(get_stats))
            .route("/api/health", get(health_check))
            .route("/api/export/json", get(export_json))
            .route("/api/export/csv", get(export_csv))
            .route("/api/update", post(update_data))
            .with_state(self.data.clone())
    }

    pub async fn update_data(&self, new_data: Vec<ScrapedData>) -> Result<()> {
        let mut data = self.data.write().await;
        *data = new_data;
        log::info!("API data updated with {} items", data.len());
        Ok(())
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
    State(data): State<SharedData>,
    Query(params): Query<SearchQuery>,
) -> (StatusCode, Json<Vec<ScrapedData>>) {
    let data_guard = data.read().await;
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

    (StatusCode::OK, Json(paginated_results))
}

async fn search_data(
    State(data): State<SharedData>,
    Query(params): Query<SearchQuery>,
) -> (StatusCode, Json<Vec<ScrapedData>>) {
    let data_guard = data.read().await;
    
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

async fn get_sources(State(data): State<SharedData>) -> (StatusCode, Json<Vec<String>>) {
    let data_guard = data.read().await;
    let mut sources: Vec<String> = data_guard
        .iter()
        .map(|item| item.source.clone())
        .collect();
    
    sources.sort();
    sources.dedup();
    
    (StatusCode::OK, Json(sources))
}

async fn get_stats(State(data): State<SharedData>) -> (StatusCode, Json<HashMap<String, usize>>) {
    let data_guard = data.read().await;
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

async fn export_json(State(data): State<SharedData>) -> (StatusCode, Json<Vec<ScrapedData>>) {
    let data_guard = data.read().await;
    (StatusCode::OK, Json(data_guard.clone()))
}

async fn export_csv(State(data): State<SharedData>) -> (StatusCode, String) {
    let data_guard = data.read().await;
    
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
    State(data): State<SharedData>,
    Json(new_data): Json<Vec<ScrapedData>>,
) -> (StatusCode, Json<HashMap<&'static str, String>>) {
    let mut data_guard = data.write().await;
    let count = new_data.len();
    *data_guard = new_data;

    let mut response = HashMap::new();
    response.insert("status", "success".to_string());
    response.insert("message", "Data updated successfully".to_string());
    response.insert("items_count", count.to_string());

    (StatusCode::OK, Json(response))
}
