mod openai;
mod provider;

use crate::{ImagePrompt, LvmImage};
use anyhow::Result;
use provider::TextToImageProvider;

pub enum LvmProviders {
    OpenAi(LvmProviderConfig),
    StableDiffusion(LvmProviderConfig),
    XAi(LvmProviderConfig),
}

pub struct LvmProviderConfig {
    pub model: String,
    pub height: u32,
    pub width: u32,
    pub num_images: u8,
}

impl Default for LvmProviderConfig {
    fn default() -> Self {
        LvmProviderConfig {
            model: String::default(),
            height: 1024,
            width: 1024,
            num_images: 1,
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
            LvmProviders::StableDiffusion(_) => todo!(),
            LvmProviders::XAi(_) => todo!(),
        }
    }
}
