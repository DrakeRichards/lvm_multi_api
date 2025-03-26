use lvm_multi_api::{
    ImagePrompt, LvmImage, LvmProviders, ProviderConfiguration, TextToImageRequest,
    TextToImageRequestExtendedParameters,
};
use tokio::runtime::Runtime;

/// Generate an image given a text input using OpenAI's DALL-E 3 model
#[test]
fn test_t2i_openai() {
    let config = ProviderConfiguration::default();
    let prompt: ImagePrompt = ImagePrompt {
        positive: Some("A painting of a cat".to_string()),
        negative: None,
    };
    let request = TextToImageRequest {
        model: Some("dall-e-3".to_string()),
        prompt,
        ..Default::default()
    };
    let provider = LvmProviders::OpenAi(config);
    let rt = Runtime::new().unwrap();
    let image: Vec<LvmImage> = rt
        .block_on(async move { provider.text_to_image(request).await })
        .unwrap();
    assert!(!image.first().unwrap().data.is_empty());
}

/// Generate an image given a text input using a local Automatic1111 instance
#[test]
fn test_t2i_automatic1111() {
    let config = ProviderConfiguration {
        base_url: Some("http://localhost:7860".to_string()),
        ..Default::default()
    };
    let prompt: ImagePrompt = ImagePrompt {
        positive: Some("A painting of a cat".to_string()),
        negative: Some("dog".to_string()),
    };
    let request = TextToImageRequest {
        model: Some("sd_xl_base_1.0".to_string()),
        prompt,
        extended: Some(TextToImageRequestExtendedParameters {
            steps: Some(10),
            sampler_name: Some("UniPC".to_string()),
            cfg_scale: Some(3.0),
            ..Default::default()
        }),
        ..Default::default()
    };
    let provider = LvmProviders::Automatic1111(config);
    let rt = Runtime::new().unwrap();
    let image: Vec<LvmImage> = rt
        .block_on(async move { provider.text_to_image(request).await })
        .unwrap();
    assert!(!image.first().unwrap().data.is_empty());
}

/// Generate multiple images given a text input using a local Automatic1111 instance
#[test]
fn test_t2i_automatic1111_multiple() {
    let config = ProviderConfiguration {
        base_url: Some("http://localhost:7860".to_string()),
        ..Default::default()
    };
    let prompt: ImagePrompt = ImagePrompt {
        positive: Some("A painting of a cat".to_string()),
        negative: Some("dog".to_string()),
    };
    let request = TextToImageRequest {
        model: Some("sd_xl_base_1.0".to_string()),
        prompt,
        num_batches: Some(2),
        extended: Some(TextToImageRequestExtendedParameters {
            steps: Some(10),
            sampler_name: Some("UniPC".to_string()),
            cfg_scale: Some(3.0),
            batch_size: Some(1),
            ..Default::default()
        }),
        ..Default::default()
    };
    let provider = LvmProviders::Automatic1111(config);
    let rt = Runtime::new().unwrap();
    let images: Vec<LvmImage> = rt
        .block_on(async move { provider.text_to_image(request).await })
        .unwrap();
    assert!(!images.first().unwrap().data.is_empty());
    assert!(!images.last().unwrap().data.is_empty());
    assert_eq!(images.len(), 2);
}

/// Generate an image given a text input using XAI
#[test]
fn test_t2i_xai() {
    let config = ProviderConfiguration::default();
    let prompt: ImagePrompt = ImagePrompt {
        positive: Some("A painting of a cat".to_string()),
        negative: None,
    };
    let request = TextToImageRequest {
        model: Some("grok-2-image".to_string()),
        prompt,
        ..Default::default()
    };
    let provider = LvmProviders::XAi(config);
    let rt = Runtime::new().unwrap();
    let image: Vec<LvmImage> = rt
        .block_on(async move { provider.text_to_image(request).await })
        .unwrap();
    assert!(!image.first().unwrap().data.is_empty());
}
