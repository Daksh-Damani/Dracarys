use anyhow::Result;

use super::provider::{
    DracarysInference,
    GenerationRequest,
    GenerationResponse,
};

pub struct InferenceEngine {
    model_name: String,
    loaded: bool,
}

impl InferenceEngine {
    pub fn new(model_name: impl Into<String>) -> Self {
        Self {
            model_name: model_name.into(),
            loaded: false,
        }
    }

    pub fn load(&mut self) -> Result<()> {
        println!("Loading model: {}", self.model_name);

        // Actual model loading will be implemented next.
        self.loaded = true;

        Ok(())
    }
}

#[async_trait::async_trait]
impl DracarysInference for InferenceEngine {
    async fn generate(
        &self,
        request: GenerationRequest,
    ) -> Result<GenerationResponse> {
        if !self.loaded {
            anyhow::bail!("Inference engine is not loaded");
        }

        Ok(GenerationResponse {
            text: format!(
                "DRACARYS received: {}",
                request.prompt
            ),
            tokens_generated: 0,
        })
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }

    fn is_loaded(&self) -> bool {
        self.loaded
    }
}