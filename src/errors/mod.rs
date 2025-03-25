use crate::parameters::provider::ProviderConfiguration;

#[derive(Debug)]
pub enum LvmError {}

/// Something went wrong while trying to configure a provider.
#[derive(Debug)]
pub struct ProviderConfigurationError {
    pub message: String,
    pub configuration: ProviderConfiguration,
}

impl std::fmt::Display for ProviderConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Provider configuration error: {}. Configuration: {:?}",
            self.message, self.configuration
        )
    }
}
