use clap::ValueEnum;

#[derive(Debug, ValueEnum, Clone)]
pub enum CliLvmProviders {
    OpenAi,
    Automatic1111,
    XAi,
}
