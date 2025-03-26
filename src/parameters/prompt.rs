#[cfg(feature = "clap")]
use clap::Args;

/// A prompt for an image generation request.
#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "clap", derive(Args))]
pub struct ImagePrompt {
    pub positive_prompt: Option<String>,
    pub negative_prompt: Option<String>,
}
