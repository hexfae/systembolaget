pub type Result<T, E = Error> = color_eyre::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("there is no api key")]
    NoApiKey,
    #[error("could not fetch api key")]
    FetchApiKey,
    #[error("invalid regex")]
    Regex(#[from] regex::Error),
    #[error("network issue")]
    Network(#[from] reqwest::Error),
    #[error("deserialization issue")]
    Deserialize(#[from] serde_json::Error),
    #[error("i/o issue")]
    IO(#[from] std::io::Error),
    #[error("parsing issue")]
    Strum(#[from] strum::ParseError),
    #[error("inquire issue")]
    Inquire(#[from] inquire::InquireError),
    #[error(transparent)]
    Other(#[from] color_eyre::Report),
}
