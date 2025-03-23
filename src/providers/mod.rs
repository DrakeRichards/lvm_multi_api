pub mod automatic1111;
pub mod openai;
mod provider;

use crate::{ImagePrompt, LvmImage};
use anyhow::Result;
use provider::TextToImageProvider;

#[derive(Clone)]
pub enum LvmProviders {
    OpenAi(LvmProviderConfig),
    Automatic1111(LvmProviderConfig),
    XAi(LvmProviderConfig),
}

#[derive(Clone)]
pub struct LvmProviderConfig {
    pub model: String,
    pub height: u32,
    pub width: u32,
    pub num_images: u8,
    pub base_url: Option<String>,
    pub steps: Option<u32>,
    pub sampler_name: Option<String>,
    pub cfg_scale: Option<f64>,
}

impl Default for LvmProviderConfig {
    fn default() -> Self {
        LvmProviderConfig {
            model: String::default(),
            height: 1024,
            width: 1024,
            num_images: 1,
            base_url: None,
            steps: None,
            sampler_name: None,
            cfg_scale: None,
        }
    }
}

impl LvmProviders {
    pub async fn text_to_image(&self, prompt: &ImagePrompt) -> Result<Vec<LvmImage>> {
        match self {
            LvmProviders::OpenAi(config) => {
                openai::OpenAiProvider::from(config)
                    .text_to_image(prompt)
                    .await
            }
            LvmProviders::Automatic1111(config) => {
                automatic1111::Automatic1111Provider::from(config)
                    .text_to_image(prompt)
                    .await
            }
            LvmProviders::XAi(_) => todo!(),
        }
    }
}
