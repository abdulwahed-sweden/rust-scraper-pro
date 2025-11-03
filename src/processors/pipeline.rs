use crate::{
    core::models::ScrapedData,
    processors::{deduplicator::Deduplicator, normalizer::Normalizer, validator::Validator},
    utils::error::ScraperError,
};
use anyhow::Result;

pub struct ProcessingPipeline {
    validators: Vec<Validator>,
    normalizers: Vec<Normalizer>,
    deduplicators: Vec<Deduplicator>,
}

impl ProcessingPipeline {
    pub fn new() -> Self {
        Self {
            validators: vec![Validator::new()],
            normalizers: vec![Normalizer::new()],
            deduplicators: vec![Deduplicator::new()],
        }
    }

    pub async fn process(&self, mut data: Vec<ScrapedData>) -> Result<Vec<ScrapedData>> {
        log::info!("Processing {} items through pipeline", data.len());

        // Validate
        for validator in &self.validators {
            data = validator.validate(data).await?;
        }

        // Normalize
        for normalizer in &self.normalizers {
            data = normalizer.normalize(data).await?;
        }

        // Deduplicate
        for deduplicator in &self.deduplicators {
            data = deduplicator.deduplicate(data).await?;
        }

        log::info!("Pipeline processing completed: {} items remaining", data.len());
        Ok(data)
    }

    pub fn add_validator(&mut self, validator: Validator) {
        self.validators.push(validator);
    }

    pub fn add_normalizer(&mut self, normalizer: Normalizer) {
        self.normalizers.push(normalizer);
    }

    pub fn add_deduplicator(&mut self, deduplicator: Deduplicator) {
        self.deduplicators.push(deduplicator);
    }
}

impl Default for ProcessingPipeline {
    fn default() -> Self {
        Self::new()
    }
}
