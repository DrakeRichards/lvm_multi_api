//! Implementations common to all providers.

use crate::{
    images::LvmImage, parameters::provider::ProviderConfiguration,
    parameters::text_to_image::TextToImageRequest, traits::TextToImageProvider,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[cfg(feature = "automatic1111")]
use crate::providers::automatic1111::Automatic1111Provider;
#[cfg(feature = "openai")]
use crate::providers::openai::OpenAiProvider;
#[cfg(feature = "xai")]
use crate::providers::xai::XAiProvider;

/// Supported LVM providers.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum LvmProviders {
    #[cfg(feature = "openai")]
    OpenAi(ProviderConfiguration),
    #[cfg(feature = "automatic1111")]
    Automatic1111(ProviderConfiguration),
    #[cfg(feature = "xai")]
    XAi(ProviderConfiguration),
}

impl Default for LvmProviders {
    #[allow(unreachable_code, clippy::needless_return, clippy::panic)]
    fn default() -> Self {
        #[cfg(feature = "openai")]
        return LvmProviders::OpenAi(ProviderConfiguration::default());
        #[cfg(feature = "xai")]
        return LvmProviders::XAi(ProviderConfiguration::default());
        #[cfg(feature = "automatic1111")]
        return LvmProviders::Automatic1111(ProviderConfiguration::default());
        panic!("No provider feature enabled");
    }
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
