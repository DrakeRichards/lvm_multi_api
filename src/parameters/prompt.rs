use serde::{Deserialize, Serialize};

#[cfg(feature = "clap")]
use clap::Args;

/// A prompt for an image generation request.
#[derive(Debug, Deserialize, Default, Serialize, PartialEq, Clone)]
#[cfg_attr(feature = "clap", derive(Args))]
pub struct ImagePrompt {
    /// The positive prompt to use for image generation.
    #[cfg_attr(feature = "clap", arg(long))]
    pub positive_prompt: Option<String>,
    /// The negative prompt to use for image generation. Not supported by all providers or models.
    #[cfg_attr(feature = "clap", arg(long))]
    pub negative_prompt: Option<String>,
}
