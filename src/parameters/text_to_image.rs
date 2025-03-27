use crate::parameters::prompt::ImagePrompt;
use serde::{Deserialize, Serialize};

#[cfg(feature = "clap")]
use clap::Args;

/// A request to generate an image from text.
#[derive(Debug, Deserialize, Default, Serialize, PartialEq, Clone)]
#[cfg_attr(feature = "clap", derive(Args))]
pub struct TextToImageRequest {
    #[cfg_attr(feature = "clap", clap(flatten))]
    pub prompt: ImagePrompt,
    /// The model to use for image generation.
    #[cfg_attr(feature = "clap", arg(long))]
    pub model: Option<String>,
    /// The height of the image in pixels.
    #[cfg_attr(feature = "clap", arg(long))]
    pub height: Option<u32>,
    /// The width of the image in pixels.
    #[cfg_attr(feature = "clap", arg(long))]
    pub width: Option<u32>,
    /// The number of image batches to process. Most providers do not support multi-image batches, so for them this is equivalent to the number of images.
    #[cfg_attr(feature = "clap", arg(long))]
    pub num_batches: Option<u32>,
    #[cfg_attr(feature = "clap", clap(flatten))]
    pub extended: Option<TextToImageRequestExtendedParameters>,
}

/// Additional parameters used by some providers.
#[derive(Debug, Deserialize, Default, Serialize, PartialEq, Clone)]
#[cfg_attr(feature = "clap", derive(Args))]
pub struct TextToImageRequestExtendedParameters {
    /// How many images to generate per batch.
    #[cfg_attr(feature = "clap", arg(long))]
    pub batch_size: Option<u32>,
    /// The number of steps to use for image generation.
    #[cfg_attr(feature = "clap", arg(long))]
    pub steps: Option<u32>,
    /// The name of the sampler
    #[cfg_attr(feature = "clap", arg(long))]
    pub sampler_name: Option<String>,
    /// The CFG scale
    #[cfg_attr(feature = "clap", arg(long))]
    pub cfg_scale: Option<f64>,
    /// The name of the VAE model
    #[cfg_attr(feature = "clap", arg(long))]
    pub vae: Option<String>,
    /// The seed to use for image generation.
    #[cfg_attr(feature = "clap", arg(long))]
    pub seed: Option<u32>,
}
