mod providers;

use crate::TextToImageRequest;
use clap::Parser;
use providers::CliLvmProviders;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// The provider to use
    pub provider: CliLvmProviders,

    /// The request to send to the provider
    #[clap(flatten)]
    pub request: TextToImageRequest,
}
