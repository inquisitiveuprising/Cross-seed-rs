#[derive(Debug)]
pub enum ClientError {
    HttpError(reqwest::Error),
    SearchResultError(super::ResultError)
}

impl From<reqwest::Error> for ClientError {
    fn from(e: reqwest::Error) -> Self {
        ClientError::HttpError(e)
    }
}

impl From<super::ResultError> for ClientError {
    fn from(e: super::ResultError) -> Self {
        ClientError::SearchResultError(e)
    }
}