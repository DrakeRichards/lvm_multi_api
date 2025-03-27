use serde::{Deserialize, Serialize};

#[cfg(feature = "clap")]
use clap::Args;

/// Parameters common to most LVM providers.
#[derive(Debug, Deserialize, Default, Serialize, PartialEq, Clone)]
#[cfg_attr(feature = "clap", derive(Args))]
pub struct ProviderConfiguration {
    /// The base URL of the provider.
    #[cfg_attr(feature = "clap", arg(long))]
    pub base_url: Option<String>,
    /// The name of the API key environment variable.
    #[cfg_attr(feature = "clap", arg(long))]
    pub api_key_env_var: Option<String>,
}
