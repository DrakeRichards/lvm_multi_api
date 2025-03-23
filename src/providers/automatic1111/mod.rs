pub mod api;

use crate::{
    ImagePrompt, LvmImage,
    providers::{LvmProviderConfig, TextToImageProvider},
};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct Automatic1111Provider {
    pub base_url: String,
    pub model: String,
    pub steps: u32,
    pub sampler_name: String,
    pub cfg_scale: f64,
    pub width: u32,
    pub height: u32,
    pub num_images: u8,
}

impl Default for Automatic1111Provider {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:7860".to_string(),
            model: "UniPC".to_string(),
            steps: 10,
            sampler_name: "UniPC".to_string(),
            cfg_scale: 3.0,
            width: 1024,
            height: 1024,
            num_images: 1,
        }
    }
}

impl From<&LvmProviderConfig> for Automatic1111Provider {
    /// Create a new Automatic1111Provider from a LvmProviderConfig.
    /// Also fills in the missing fields with default values.
    /// These can be overridden with the `with_extra_config` method.
    fn from(config: &LvmProviderConfig) -> Self {
        Automatic1111Provider {
            base_url: "http://localhost:7860".to_string(),
            model: config.model.clone(),
            steps: config.steps.unwrap_or(10),
            sampler_name: config.sampler_name.clone().unwrap_or("UniPC".to_string()),
            cfg_scale: config.cfg_scale.unwrap_or(3.0),
            width: config.width,
            height: config.height,
            num_images: config.num_images,
        }
    }
}

/// A provider for generating images with a local Stable Diffusion instance.
#[derive(Deserialize, Debug, Serialize)]
pub struct StableDiffusionXLProvider {
    /// The URL for the local Stable Diffusion instance.
    pub url: String,
}

impl Default for StableDiffusionXLProvider {
    fn default() -> Self {
        Self {
            url: "http://localhost:7860".to_string(),
        }
    }
}

#[async_trait]
impl TextToImageProvider for Automatic1111Provider {
    /// Generate images from text prompts using the Automatic1111 provider.
    async fn text_to_image(&self, prompt: &ImagePrompt) -> Result<Vec<LvmImage>> {
        self.queue_txt2img(prompt.clone()).await
    }
}
