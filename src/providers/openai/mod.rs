use std::ops::Deref;

use crate::{
    images::LvmImage,
    parameters::{provider::ProviderConfiguration, text_to_image::TextToImageRequest},
    traits::TextToImageProvider,
};
use anyhow::Result;
use async_openai::{
    Client,
    types::{CreateImageRequestArgs, ImageModel, ImageResponseFormat, ImageSize},
};
use async_trait::async_trait;
use dotenvy::dotenv;

// There's nothing to configure on a provider level for OpenAI.
pub struct OpenAiProvider {}

fn to_openai_size(width: Option<u32>, height: Option<u32>) -> ImageSize {
    if let (Some(width), Some(height)) = (width, height) {
        match (width, height) {
            (1024, 1024) => ImageSize::S1024x1024,
            (1024, 1792) => ImageSize::S1024x1792,
            (1792, 1024) => ImageSize::S1792x1024,
            (256, 256) => ImageSize::S256x256,
            (512, 512) => ImageSize::S512x512,
            _ => ImageSize::S1024x1024,
        }
    } else {
        ImageSize::S1024x1024
    }
}

fn to_openai_model(model: Option<String>) -> ImageModel {
    model.map_or(ImageModel::DallE2, |model| match model {
        model if model.eq_ignore_ascii_case("dall-e-2") => ImageModel::DallE2,
        model if model.eq_ignore_ascii_case("dall-e-3") => ImageModel::DallE3,
        _ => ImageModel::DallE2,
    })
}

fn to_openai_batch_size(num_batches: Option<u32>) -> u8 {
    num_batches.map_or(1, |num_batches| {
        if num_batches > 0 && num_batches < 256 {
            num_batches as u8
        } else {
            eprintln!("Invalid number of batches. Using default value of 1.");
            1
        }
    })
}

#[async_trait]
impl TextToImageProvider for OpenAiProvider {
    async fn text_to_image(&self, request: TextToImageRequest) -> Result<Vec<LvmImage>> {
        // Load environment variables from a .env file.
        dotenv()?;

        // Check if the API key environment variable is set.
        std::env::var("OPENAI_API_KEY")
            .map_err(|_| anyhow::anyhow!("OPENAI_API_KEY environment variable not set."))?;

        // Create a new OpenAI client.
        let client = Client::new();

        // Create the request.
        let request = CreateImageRequestArgs::default()
            .model(to_openai_model(request.model))
            .prompt(request.prompt.positive_prompt.unwrap_or_default())
            .n(to_openai_batch_size(request.num_batches))
            .response_format(ImageResponseFormat::B64Json)
            .size(to_openai_size(request.width, request.height))
            .build()?;

        // Send the request to OpenAI's API.
        let response = client.images().create(request).await?;

        Ok(response
            .data
            .into_iter()
            .map(|image| image.deref().clone().into())
            .collect())
    }
}

impl From<&ProviderConfiguration> for OpenAiProvider {
    fn from(_config: &ProviderConfiguration) -> Self {
        OpenAiProvider {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_openai_size() {
        assert_eq!(
            to_openai_size(Some(1024), Some(1024)),
            ImageSize::S1024x1024
        );
        assert_eq!(
            to_openai_size(Some(1024), Some(1792)),
            ImageSize::S1024x1792
        );
        assert_eq!(
            to_openai_size(Some(1792), Some(1024)),
            ImageSize::S1792x1024
        );
        assert_eq!(to_openai_size(Some(256), Some(256)), ImageSize::S256x256);
        assert_eq!(to_openai_size(Some(512), Some(512)), ImageSize::S512x512);
    }
}
