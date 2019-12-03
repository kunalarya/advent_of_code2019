use std::error::Error;
use std::fmt;

pub type Res<T> = Result<T, Box<dyn Error>>;

/*
 * Generic error handling.
 */
#[derive(Debug)]
pub struct GenericError {
    msg: String,
}

impl GenericError {
    fn new(msg: &str) -> GenericError {
        GenericError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for GenericError {
    fn description(&self) -> &str {
        &self.msg
    }
}

pub fn error<T>(msg: &str) -> Res<T> {
    Err(Box::new(GenericError::new(msg)))
}
