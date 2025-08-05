pub type Result<T> = std::result::Result<T, FixedFastError>;

#[derive(Debug, thiserror::Error)]
pub enum FixedFastError {
    #[error("{0} is out of range")]
    OutOfRange(i128),
    #[error("domain error: {0}")]
    DomainError(&'static str),
    #[error("attempted to divide by zero")]
    DivideByZero,
    #[error("arithmetic overflow")]
    Overflow,
}

// Provide automatic conversion from core int errors if needed
impl From<std::num::TryFromIntError> for FixedFastError {
    fn from(_: std::num::TryFromIntError) -> Self {
        FixedFastError::DomainError("integer conversion error")
    }
}
