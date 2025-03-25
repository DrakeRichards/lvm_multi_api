/// Parameters common to most LVM providers.
#[derive(Clone, Default)]
pub struct ProviderConfiguration {
    /// The base URL of the provider.
    pub base_url: Option<String>,
    /// The name of the API key environment variable.
    pub api_key_env_var: Option<String>,
}
