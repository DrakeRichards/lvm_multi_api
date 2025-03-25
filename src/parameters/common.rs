/// Parameters common to all types of image generation requests.
#[derive(Clone)]
pub struct RequestParameters {
    /// The name of the model to use.
    pub model: String,
    /// The height in pixels of the generated image.
    pub height: u32,
    /// The width in pixels of the generated image.
    pub width: u32,
    /// How many batches of images to generate.
    /// For most providers, each batch will contain one image,
    /// so this is equivalent to the number of images to generate.
    pub num_batches: u32,
    /// Additional parameters used by some providers.
    pub extended: Option<RequestParametersExtended>,
}

impl Default for RequestParameters {
    fn default() -> Self {
        RequestParameters {
            model: String::default(),
            height: 1024,
            width: 1024,
            num_batches: 1,
            extended: None,
        }
    }
}

/// Additional parameters used by some providers.
#[derive(Clone)]
pub struct RequestParametersExtended {
    pub base_url: String,
    pub batch_size: Option<u32>,
    pub steps: Option<u32>,
    pub sampler_name: Option<String>,
    pub cfg_scale: Option<f64>,
    pub checkpoint: Option<String>,
    pub vae: Option<String>,
    pub callback_url: Option<String>,
}
