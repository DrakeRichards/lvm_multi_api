//! Endpoints and types for interacting with the Stable Diffusion API.

pub mod config;
pub mod model;
pub mod queue;
pub mod status;
pub mod txt2img;

use super::Automatic1111Provider;
use serde::Serialize;

/// Types of requests that can be sent to the Stable Diffusion API to generate images.
/// Right now only supports Txt2Img requests. Other potential requests include Img2Img and Control.
#[derive(Debug, Serialize)]
pub enum RequestBody {
    Txt2Img(txt2img::Txt2ImgRequestBody),
}
