use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("TypeError: {0}")]
    TypeError(String),
    #[error("InternalServerError: {0}")]
    InternalServerError(String),
}

impl MyError {
    pub fn type_error(v: &str) -> MyError {
        MyError::TypeError(v.to_string())
    }
}
