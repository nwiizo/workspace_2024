// src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum JPostError {
    #[error("Invalid postal code format")]
    InvalidFormat,
    #[error("Address not found")]
    NotFound,
}
