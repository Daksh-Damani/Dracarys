use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct GenerationRequest {
    pub prompt: String,
    pub max_tokens: usize,
    pub temperature: f32,
}

#[derive(Debug, Clone)]
pub struct GenerationResponse {
    pub text: String,
    pub tokens_generated: usize,
}

#[async_trait]
pub trait DracarysInference: Send + Sync {
    async fn generate(
        &self,
        request: GenerationRequest,
    ) -> Result<GenerationResponse>;

    fn model_name(&self) -> &str;

    fn is_loaded(&self) -> bool;
}