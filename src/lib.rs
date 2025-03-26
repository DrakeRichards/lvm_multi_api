#![deny(unused_crate_dependencies)]

mod errors;
mod images;
mod parameters;
mod providers;
mod traits;

#[cfg(feature = "clap")]
pub mod cli;

pub use errors::LvmError;
pub use images::LvmImage;
pub use parameters::{
    prompt::ImagePrompt,
    provider::ProviderConfiguration,
    text_to_image::{TextToImageRequest, TextToImageRequestExtendedParameters},
};
pub use providers::LvmProviders;

#[cfg(test)]
mod tests {
    use tokio as _;
}
