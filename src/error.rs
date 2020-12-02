use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, AocError>;

/// AocError enumerates all possible errors returned by this library
#[derive(Debug)]
pub enum AocError {
    PasswordDefinitionError(String),
    PolicyDefinitionError(String),
    MyErrorWithMessage(String),

    /// Represents all other cases of
    IOError(std::io::Error),

    /// Represents all other cases of
    ParseIntError(ParseIntError),
}

impl std::error::Error for AocError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            AocError::PasswordDefinitionError(_) => None,
            AocError::PolicyDefinitionError(_) => None,
            AocError::MyErrorWithMessage(_) => None,
            AocError::IOError(ref err) => Some(err),
            AocError::ParseIntError(ref err) => Some(err),
        }
    }
}

impl std::fmt::Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            AocError::PasswordDefinitionError(ref def) => {
                write!(f, "Invalid password definition: '{}'", def)
            }
            AocError::PolicyDefinitionError(ref def) => {
                write!(f, "Invalid policy definition: '{}'", def)
            }
            AocError::MyErrorWithMessage(ref msg) => {
                write!(f, "some error with msg '{}'", msg)
            },
            AocError::IOError(ref err) => err.fmt(f),
            AocError::ParseIntError(ref err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for AocError {
    fn from(err: std::io::Error) -> AocError {
        AocError::IOError(err)
    }
}

impl From<ParseIntError> for AocError {
    fn from(err: ParseIntError) -> AocError {
        AocError::ParseIntError(err)
    }
}
