//! Status API for Stable Diffusion XL.

use super::Automatic1111Provider;
use anyhow::Result;
use reqwest::get;

impl Automatic1111Provider {
    /// Send a GET request to `/sdapi/v1/status` to check if the local Stable Diffusion instance is up.
    pub async fn is_up(&self) -> Result<bool> {
        let response = get(&self.base_url).await?;
        Ok(response.status().is_success())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial(stable_diffusion)]
    async fn test_is_up() {
        let provider = Automatic1111Provider::default();
        let is_up = provider.is_up().await.unwrap();
        assert!(is_up);
    }
}
