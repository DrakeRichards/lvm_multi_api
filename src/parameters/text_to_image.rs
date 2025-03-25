use crate::parameters::prompt::ImagePrompt;

#[derive(Clone, Default)]
pub struct TextToImageRequest {
    pub prompt: ImagePrompt,
    pub model: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub num_batches: Option<u32>,
    pub extended: Option<TextToImageRequestExtendedParameters>,
}

/// Additional parameters used by some providers.
#[derive(Clone, Default)]
pub struct TextToImageRequestExtendedParameters {
    pub batch_size: Option<u32>,
    pub steps: Option<u32>,
    pub sampler_name: Option<String>,
    pub cfg_scale: Option<f64>,
    pub checkpoint: Option<String>,
    pub vae: Option<String>,
    pub callback_url: Option<String>,
}
