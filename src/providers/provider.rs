use crate::{ImagePrompt, LvmImage};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TextToImageProvider {
    /// Generate an image given a text input using the provider's model.
    /// Other configuration should be set using the `self` object
    async fn text_to_image(&self, prompt: &ImagePrompt) -> Result<Vec<LvmImage>>;
}
