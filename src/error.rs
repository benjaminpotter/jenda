use std::fmt;

pub enum JendaError {}

impl fmt::Display for JendaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, ""),
        }
    }
}
