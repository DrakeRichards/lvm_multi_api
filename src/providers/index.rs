//! Implementations common to all providers.

use crate::{
    images::LvmImage,
    parameters::provider::ProviderConfiguration,
    parameters::text_to_image::TextToImageRequest,
    providers::{automatic1111::Automatic1111Provider, openai::OpenAiProvider},
    traits::TextToImageProvider,
};
use anyhow::Result;

/// Supported LVM providers.
#[derive(Clone)]
pub enum LvmProviders {
    OpenAi(ProviderConfiguration),
    Automatic1111(ProviderConfiguration),
    XAi(ProviderConfiguration),
}

impl LvmProviders {
    pub async fn text_to_image(&self, request: TextToImageRequest) -> Result<Vec<LvmImage>> {
        match self {
            LvmProviders::OpenAi(config) => {
                OpenAiProvider::from(config).text_to_image(request).await
            }
            LvmProviders::Automatic1111(config) => {
                Automatic1111Provider::from(config)
                    .text_to_image(request)
                    .await
            }
            LvmProviders::XAi(_) => todo!(),
        }
    }
}
