use crate::parameters::prompt::ImagePrompt;

#[cfg(feature = "clap")]
use clap::Args;

/// A request to generate an image from text.
#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "clap", derive(Args))]
pub struct TextToImageRequest {
    #[cfg_attr(feature = "clap", clap(flatten))]
    pub prompt: ImagePrompt,
    pub model: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub num_batches: Option<u32>,
    #[cfg_attr(feature = "clap", clap(flatten))]
    pub extended: Option<TextToImageRequestExtendedParameters>,
}

/// Additional parameters used by some providers.
#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "clap", derive(Args))]
pub struct TextToImageRequestExtendedParameters {
    pub batch_size: Option<u32>,
    pub steps: Option<u32>,
    pub sampler_name: Option<String>,
    pub cfg_scale: Option<f64>,
    pub checkpoint: Option<String>,
    pub vae: Option<String>,
    pub callback_url: Option<String>,
}
