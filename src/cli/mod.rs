use crate::{LvmProviders, ProviderConfiguration, TextToImageRequest};
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// The provider to use
    #[clap(long, default_value_t = CliLvmProviders::OpenAi)]
    pub provider: CliLvmProviders,

    // Configuration for the provider
    #[clap(flatten)]
    pub provider_configuration: ProviderConfiguration,

    /// The request to send to the provider
    #[clap(flatten)]
    pub request: TextToImageRequest,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum CliLvmProviders {
    OpenAi,
    Automatic1111,
    XAi,
}

impl std::fmt::Display for CliLvmProviders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliLvmProviders::OpenAi => write!(f, "open-ai"),
            CliLvmProviders::Automatic1111 => write!(f, "automatic1111"),
            CliLvmProviders::XAi => write!(f, "x-ai"),
        }
    }
}

impl Cli {
    pub fn get_provider(&self) -> LvmProviders {
        match self.provider {
            CliLvmProviders::OpenAi => LvmProviders::OpenAi(self.provider_configuration.clone()),
            CliLvmProviders::Automatic1111 => {
                LvmProviders::Automatic1111(self.provider_configuration.clone())
            }
            CliLvmProviders::XAi => LvmProviders::XAi(self.provider_configuration.clone()),
        }
    }
}
