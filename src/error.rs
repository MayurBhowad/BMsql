#[derive(Debug)]
pub enum BmsqlError {
    Io(std::io::Error),
    InvalidInput(String),
}

impl From<std::io::Error> for BmsqlError {
    fn from(error: std::io::Error) -> Self {
        BmsqlError::Io(error)
    }
}