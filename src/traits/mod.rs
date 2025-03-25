use crate::{images::LvmImage, parameters::text_to_image::TextToImageRequest};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TextToImageProvider {
    /// Generate an image given a text input using the provider's model.
    /// Other configuration should be set using the `self` object
    async fn text_to_image(&self, request: TextToImageRequest) -> Result<Vec<LvmImage>>;
}
