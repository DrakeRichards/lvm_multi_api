//! Implementations common to all providers.

use crate::{
    images::LvmImage, parameters::provider::ProviderConfiguration,
    parameters::text_to_image::TextToImageRequest, traits::TextToImageProvider,
};
use anyhow::Result;

#[cfg(feature = "automatic1111")]
use crate::providers::automatic1111::Automatic1111Provider;
#[cfg(feature = "openai")]
use crate::providers::openai::OpenAiProvider;
#[cfg(feature = "xai")]
use crate::providers::xai::XAiProvider;

/// Supported LVM providers.
#[derive(Clone)]
pub enum LvmProviders {
    #[cfg(feature = "openai")]
    OpenAi(ProviderConfiguration),
    #[cfg(feature = "automatic1111")]
    Automatic1111(ProviderConfiguration),
    #[cfg(feature = "xai")]
    XAi(ProviderConfiguration),
}

impl LvmProviders {
    pub async fn text_to_image(&self, request: TextToImageRequest) -> Result<Vec<LvmImage>> {
        match self {
            #[cfg(feature = "openai")]
            LvmProviders::OpenAi(config) => {
                OpenAiProvider::from(config).text_to_image(request).await
            }
            #[cfg(feature = "automatic1111")]
            LvmProviders::Automatic1111(config) => {
                Automatic1111Provider::from(config)
                    .text_to_image(request)
                    .await
            }
            #[cfg(feature = "xai")]
            LvmProviders::XAi(config) => XAiProvider::from(config).text_to_image(request).await,
        }
    }
}
