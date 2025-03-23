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
