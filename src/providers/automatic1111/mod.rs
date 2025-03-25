pub mod api;

use crate::{
    LvmImage, parameters::provider::ProviderConfiguration,
    parameters::text_to_image::TextToImageRequest, traits::TextToImageProvider,
};
use anyhow::Result;
use async_trait::async_trait;

const DEFAULT_BASE_URL: &str = "http://localhost:7860";

/// A provider for generating images with the Automatic1111 instance.
/// These are the fields common to all requests to the Automatic1111 provider.
/// Most of these fields are optional since Automatic1111 will fill in the missing fields with default values set on the server.
#[derive(Debug, Clone)]
pub struct Automatic1111Provider {
    pub base_url: String,
}

impl Default for Automatic1111Provider {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }
}

impl From<&ProviderConfiguration> for Automatic1111Provider {
    /// Create a new Automatic1111Provider from a LvmProviderConfig.
    /// Also fills in the missing fields with default values.
    /// These can be overridden with the `with_extra_config` method.
    fn from(config: &ProviderConfiguration) -> Self {
        Automatic1111Provider {
            base_url: config
                .base_url
                .clone()
                .unwrap_or(DEFAULT_BASE_URL.to_string()),
        }
    }
}

#[async_trait]
impl TextToImageProvider for Automatic1111Provider {
    /// Generate images from text prompts using the Automatic1111 provider.
    async fn text_to_image(&self, request: TextToImageRequest) -> Result<Vec<LvmImage>> {
        self.queue_txt2img(request).await
    }
}
