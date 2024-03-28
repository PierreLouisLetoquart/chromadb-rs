use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChromaClientError {
    #[error("Unable to parse URL: {0}")]
    UrlParseError(url::ParseError),
    #[error("Unable to send request: {0}")]
    RequestError(reqwest::Error),
    #[error("Unable to parse response: {0}")]
    ResponseError(reqwest::Error),
    #[error("Unable to parse into json response: {0}")]
    ResponseParseError(serde_json::Error),
    #[error("Respond with a bad status: {0}")]
    ResponseStatusError(String),
    #[error("Preflight request failed, status: {0}")]
    PreflightError(String),
}
