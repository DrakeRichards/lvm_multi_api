use crate::{
    images::LvmImage,
    parameters::{provider::ProviderConfiguration, text_to_image::TextToImageRequest},
    traits::TextToImageProvider,
};
use anyhow::Result;
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{CreateImageRequestArgs, ImageModel, ImageResponseFormat},
};
use async_trait::async_trait;
use dotenvy::dotenv;
use serde_json::json;

const XAI_BASE_URL: &str = "https://api.x.ai/v1";

// There's nothing to configure on a provider level for OpenAI.
pub struct XAiProvider {}

#[derive(serde::Deserialize)]
struct ImagesResponse {
    data: Vec<async_openai::types::Image>,
}

fn to_xai_model(model: Option<String>) -> ImageModel {
    model.map_or(ImageModel::Other("grok-2-image".to_string()), |model| {
        ImageModel::Other(model)
    })
}

fn to_xai_batch_size(num_batches: Option<u32>) -> u8 {
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
impl TextToImageProvider for XAiProvider {
    async fn text_to_image(&self, request: TextToImageRequest) -> Result<Vec<LvmImage>> {
        // Load environment variables from a .env file.
        dotenv()?;

        // Get the API key from the environment.
        // If the API key is not set, return an error telling the user to set it.
        let api_key = std::env::var("XAI_API_KEY")
            .map_err(|_| anyhow::anyhow!("XAI_API_KEY environment variable not set."))?;

        // Create a new OpenAI client.
        let client = Client::with_config(
            OpenAIConfig::new()
                .with_api_base(XAI_BASE_URL)
                .with_api_key(api_key),
        );

        // Create the request.
        let request = CreateImageRequestArgs::default()
            .model(to_xai_model(request.model))
            .prompt(request.prompt.positive.unwrap_or_default())
            .n(to_xai_batch_size(request.num_batches))
            .response_format(ImageResponseFormat::B64Json)
            // The size parameter is not supported at the moment. Leave it empty.
            //.size(to_xai_size(request.width, request.height))
            .build()?;

        // Send the request to OpenAI's API.
        let response: ImagesResponse = client.images().create_byot(json!(request)).await?;

        Ok(response
            .data
            .into_iter()
            .map(|image| image.into())
            .collect())
    }
}

impl From<&ProviderConfiguration> for XAiProvider {
    fn from(_config: &ProviderConfiguration) -> Self {
        XAiProvider {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parameters::prompt::ImagePrompt, providers::LvmProviders};
    use tokio::runtime::Runtime;

    /// Generate an image given a text input using XAI
    #[test]
    fn test_t2i_xai() {
        let prompt: ImagePrompt = ImagePrompt {
            positive: Some("A painting of a cat".to_string()),
            negative: Some("dog".to_string()),
        };
        let request = TextToImageRequest {
            model: Some("grok-2-image".to_string()),
            prompt,
            num_batches: Some(1),
            ..Default::default()
        };
        let provider = LvmProviders::XAi(ProviderConfiguration::default());
        let rt = Runtime::new().unwrap();
        let images: Vec<LvmImage> = rt
            .block_on(async move { provider.text_to_image(request).await })
            .unwrap();
        assert!(!images.first().unwrap().data.is_empty());
    }
}
