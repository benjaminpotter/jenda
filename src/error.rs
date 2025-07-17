use std::fmt;

#[derive(Debug)]
pub enum JendaError {
    Database(String),
}

impl fmt::Display for JendaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JendaError::Database(e) => write!(f, "database error: {}", e),
        }
    }
}
