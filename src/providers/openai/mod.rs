use std::ops::Deref;

use crate::{
    images::LvmImage,
    parameters::ImagePrompt,
    providers::{LvmProviderConfig, TextToImageProvider},
};
use anyhow::Result;
use async_openai::{
    Client,
    types::{CreateImageRequestArgs, ImageModel, ImageResponseFormat, ImageSize},
};
use async_trait::async_trait;
use dotenvy::dotenv;

pub struct OpenAiProvider {
    pub model: ImageModel,
    pub width: u32,
    pub height: u32,
    pub num_images: u8,
}

fn to_openai_size(width: &u32, height: &u32) -> ImageSize {
    match (width, height) {
        (1024, 1024) => ImageSize::S1024x1024,
        (1024, 1792) => ImageSize::S1024x1792,
        (1792, 1024) => ImageSize::S1792x1024,
        (256, 256) => ImageSize::S256x256,
        (512, 512) => ImageSize::S512x512,
        _ => ImageSize::S1024x1024,
    }
}

#[async_trait]
impl TextToImageProvider for OpenAiProvider {
    async fn text_to_image(&self, prompt: &ImagePrompt) -> Result<Vec<LvmImage>> {
        // Load environment variables from a .env file.
        dotenv()?;

        // Create a new OpenAI client.
        let client = Client::new();

        // Standardize the image size.
        let size = to_openai_size(&self.width, &self.height);

        // Create the request.
        let request = CreateImageRequestArgs::default()
            .model(self.model.clone())
            .prompt(prompt.positive_prompt.as_deref().unwrap_or_default())
            .n(self.num_images)
            .response_format(ImageResponseFormat::B64Json)
            .size(size)
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

impl From<&LvmProviderConfig> for OpenAiProvider {
    fn from(config: &LvmProviderConfig) -> Self {
        fn to_openai_model(model: &str) -> ImageModel {
            match model {
                "dall-e-3" => ImageModel::DallE3,
                "dall-e-2" => ImageModel::DallE2,
                _ => ImageModel::DallE3,
            }
        }

        OpenAiProvider {
            model: to_openai_model(&config.model),
            width: config.width,
            height: config.height,
            num_images: config.num_images,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_openai_size() {
        assert_eq!(to_openai_size(&1024, &1024), ImageSize::S1024x1024);
        assert_eq!(to_openai_size(&1024, &1792), ImageSize::S1024x1792);
        assert_eq!(to_openai_size(&1792, &1024), ImageSize::S1792x1024);
        assert_eq!(to_openai_size(&256, &256), ImageSize::S256x256);
        assert_eq!(to_openai_size(&512, &512), ImageSize::S512x512);
    }

    #[test]
    fn test_from_lvm_provider_config() {
        let config = LvmProviderConfig {
            model: "dall-e-3".to_string(),
            width: 1024,
            height: 1024,
            num_images: 1,
        };

        let provider: OpenAiProvider = OpenAiProvider::from(&config);

        assert_eq!(provider.model, ImageModel::DallE3);
        assert_eq!(provider.width, 1024);
        assert_eq!(provider.height, 1024);
        assert_eq!(provider.num_images, 1);
    }
}
