use lvm_multi_api::{ImagePrompt, LvmImage, LvmProviderConfig, LvmProviders};
use tokio::runtime::Runtime;

/// Generate an image given a text input using OpenAI's DALL-E 3 model
#[test]
fn test_t2i_openai() {
    let prompt: ImagePrompt = ImagePrompt {
        positive_prompt: Some("A painting of a cat".to_string()),
        negative_prompt: None,
    };
    let config = LvmProviderConfig {
        model: "dall-e-3".to_string(),
        ..Default::default()
    };
    let provider = LvmProviders::OpenAi(config);
    let rt = Runtime::new().unwrap();
    let image: Vec<LvmImage> = rt
        .block_on(async move { provider.text_to_image(&prompt).await })
        .unwrap();
    assert!(!image.first().unwrap().data.is_empty());
}

/// Generate an image given a text input using a local Automatic1111 instance
#[test]
fn test_t2i_automatic1111() {
    let prompt: ImagePrompt = ImagePrompt {
        positive_prompt: Some("A painting of a cat".to_string()),
        negative_prompt: Some("dog".to_string()),
    };
    let config = LvmProviderConfig {
        model: "sd_xl_base_1.0".to_string(),
        steps: Some(10),
        sampler_name: Some("UniPC".to_string()),
        cfg_scale: Some(3.0),
        ..Default::default()
    };
    let provider = LvmProviders::Automatic1111(config);
    let rt = Runtime::new().unwrap();
    let image: Vec<LvmImage> = rt
        .block_on(async move { provider.text_to_image(&prompt).await })
        .unwrap();
    assert!(!image.first().unwrap().data.is_empty());
}
