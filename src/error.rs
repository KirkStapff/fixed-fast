pub type Result<T> = std::result::Result<T, FixedFastError>;

#[derive(Debug, thiserror::Error)]
pub enum FixedFastError {
    #[error("{0} is out of range")]
    OutOfRange(usize),
}
