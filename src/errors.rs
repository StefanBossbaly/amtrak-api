#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unable to send the request: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Unable to deserialize the received value: {0}")]
    DeserializeFailed(#[from] serde_json::error::Error),

    #[error("API returned an error response: {0}")]
    ApiErrorResponse(String),
}

#[cfg(feature = "serde_debugging")]
#[derive(Debug, thiserror::Error)]
pub enum DebuggingError {
    #[error("Unable to send the request: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Unable to deserialize the received value: {error}: {response}")]
    DeserializeFailed {
        error: serde_path_to_error::Error<serde_json::error::Error>,
        response: String,
    },

    #[error("API returned an error response: {0}")]
    ApiErrorResponse(String),
}
