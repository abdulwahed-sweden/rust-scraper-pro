pub mod core;
pub mod sources;
pub mod processors;
pub mod output;
pub mod utils;

// Re-exports for easier access
pub use core::models::ScrapedData;
pub use sources::source::{Source, SourceType};
pub use processors::pipeline::ProcessingPipeline;
pub use output::{
    json::JsonOutput,
    csv::CsvOutput,
    database::{DatabaseOutput, PostgresOutput, SqliteOutput},
    api::ApiServer,
};
pub use utils::cache::HtmlCache;

// Prelude for common imports
pub mod prelude {
    pub use crate::core::models::ScrapedData;
    pub use crate::sources::source::{Source, SourceType};
    pub use crate::processors::pipeline::ProcessingPipeline;
    pub use crate::output::{
        json::JsonOutput,
        csv::CsvOutput,
        database::{DatabaseOutput, PostgresOutput, SqliteOutput},
    };
    pub use crate::utils::cache::HtmlCache;
    pub use crate::core::config::Config;
    pub use crate::core::scraper::ScraperEngine;
    pub use crate::sources::{NewsSource, EcommerceSource, SocialSource, CustomSource};
    pub use std::sync::Arc;
}