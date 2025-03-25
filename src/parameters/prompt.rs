/// A prompt for an image generation request.
#[derive(Clone, Default)]
pub struct ImagePrompt {
    pub positive: Option<String>,
    pub negative: Option<String>,
}
