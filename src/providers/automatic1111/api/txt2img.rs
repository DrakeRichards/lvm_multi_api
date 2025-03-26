//! Txt2Img API for Stable Diffusion XL.
//! Will try to execute requests in parallel if there are multiple requests.
//! Generally don't use this: use the `queue` module instead.

//use super::Automatic1111Provider;
//use crate::{LvmImage, images::LvmImageMetadata};
//use anyhow::Result;
use serde::{Deserialize, Serialize};
//use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Txt2ImgRequestBody {
    pub prompt: String,
    pub negative_prompt: String,
    pub steps: u32,
    pub batch_size: u32,
    pub width: u32,
    pub height: u32,
    pub sampler_name: String,
    pub cfg_scale: u32,
}

impl Default for Txt2ImgRequestBody {
    fn default() -> Self {
        Self {
            prompt: "".to_string(),
            negative_prompt: "".to_string(),
            steps: 6,
            batch_size: 1,
            width: 1024,
            height: 1024,
            sampler_name: "Default".to_string(),
            cfg_scale: 2,
        }
    }
}

/*
impl Automatic1111Provider {
    /// Send a POST request to `/sdapi/v1/txt2img` to start a new image generation task.
    /// The response contains the images in base64 encoding.
    pub async fn post_txt2img(&self, request: &Txt2ImgRequestBody) -> Result<Vec<LvmImage>> {
        let endpoint = "/sdapi/v1/txt2img";
        let url = format!("{}{}", self.base_url, endpoint);
        let body = serde_json::to_string(request)?;
        let client = reqwest::Client::new();
        let response = client.post(url).body(body).send().await?;
        let response: Value = serde_json::from_str(&response.text().await?)?;
        let images: Vec<LvmImage> = response["images"]
            .as_array()
            .ok_or(anyhow::anyhow!("Unable to get images."))?
            .iter()
            .map(|image| {
                let metadata: Option<LvmImageMetadata> = Some(LvmImageMetadata {
                    generation_params: serde_json::to_string(&request).ok(),
                });
                image
                    .as_str()
                    .ok_or(anyhow::anyhow!("Unable to get image."))
                    .map(|image| LvmImage {
                        data: image.as_bytes().to_vec(),
                        metadata,
                    })
            })
            .collect::<Result<Vec<LvmImage>>>()?;
        Ok(images)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use base64::Engine;
    use serial_test::serial;

    #[tokio::test]
    #[serial(stable_diffusion, local_server)]
    async fn test_post_txt2img() {
        let provider = Automatic1111Provider::default();
        let request = Txt2ImgRequestBody {
            prompt: "A cat".to_string(),
            negative_prompt: "".to_string(),
            steps: 6,
            batch_size: 1,
            width: 1024,
            height: 1024,
            sampler_name: "DPM++ 2M".to_string(),
            cfg_scale: 2,
        };
        let received_images = provider.post_txt2img(&request).await.unwrap();
        // Assert that we get one image.
        assert_eq!(received_images.len(), 1);
        // Assert that the image is of size 1024x1024.
        let image = &received_images.first().unwrap().data;
        let image = base64::prelude::BASE64_STANDARD.decode(image).unwrap();
        let image = image::load_from_memory(&image).unwrap();
        assert_eq!(image.width(), 1024);
        assert_eq!(image.height(), 1024);
    }
}
*/
