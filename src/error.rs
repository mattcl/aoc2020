use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, AocError>;

/// AocError enumerates all possible errors returned by this library
#[derive(Debug)]
pub enum AocError {
    ForestDefinitionError(String),
    InvalidAnswers(String),
    InvalidLocator(String),
    InvalidRule(String),
    SeatNotFound(String),
    PassportInfoError(String),
    PassportInvalid(String),
    PasswordDefinitionError(String),
    PolicyDefinitionError(String),

    UnknownBag(String),

    /// Generic
    InputError(String),
    InvalidInput(String),

    /// console
    InvalidInstruction(String),
    InvalidProgram(String),
    UnknownOperation(String),

    /// XMAS
    NoOutlier,
    NoWeakness,

    /// Adapter
    NoAdapterChain,

    /// Seating
    SeatDoesNotExist((usize, usize)),

    /// Memory Game
    GameError(String),

    /// Ticket rules
    NoValidRuleOrder,

    /// Represents all other cases of
    IOError(std::io::Error),

    /// Represents all other cases of
    OsStringErr(std::ffi::OsString),

    /// Represents all other cases of
    ParseIntError(ParseIntError),
}

impl std::error::Error for AocError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            AocError::ForestDefinitionError(_) => None,
            AocError::InputError(_) => None,
            AocError::InvalidInput(_) => None,
            AocError::InvalidAnswers(_) => None,
            AocError::InvalidLocator(_) => None,
            AocError::InvalidRule(_) => None,
            AocError::SeatNotFound(_) => None,
            AocError::PassportInfoError(_) => None,
            AocError::PassportInvalid(_) => None,
            AocError::PasswordDefinitionError(_) => None,
            AocError::PolicyDefinitionError(_) => None,
            AocError::UnknownBag(_) => None,
            AocError::IOError(ref err) => Some(err),
            AocError::OsStringErr(_) => None,
            AocError::ParseIntError(ref err) => Some(err),
            AocError::InvalidInstruction(_) => None,
            AocError::InvalidProgram(_) => None,
            AocError::UnknownOperation(_) => None,
            AocError::NoOutlier => None,
            AocError::NoWeakness => None,
            AocError::NoAdapterChain => None,
            AocError::SeatDoesNotExist(_) => None,
            AocError::GameError(_) => None,
            AocError::NoValidRuleOrder => None,
        }
    }
}

impl std::fmt::Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            AocError::InputError(ref def) => write!(f, "Could not load input: '{}'", def),
            AocError::InvalidInput(ref def) => write!(f, "Invalid input: '{}'", def),
            AocError::InvalidAnswers(ref def) => write!(f, "Customs answers invalid: '{}'", def),
            AocError::InvalidLocator(ref def) => write!(f, "Invalid locator: '{}'", def),
            AocError::InvalidRule(ref def) => write!(f, "Invalid rule: '{}'", def),
            AocError::SeatNotFound(ref def) => write!(f, "Seat not found: '{}'", def),
            AocError::ForestDefinitionError(ref def) => {
                write!(f, "Invalid Forest definition: '{}'", def)
            }
            AocError::PassportInfoError(ref def) => {
                write!(f, "Cannot make passport from info: '{}'", def)
            }
            AocError::PassportInvalid(ref def) => {
                write!(f, "Invalid passport. Missing or invalid field: '{}'", def)
            }
            AocError::PasswordDefinitionError(ref def) => {
                write!(f, "Invalid password definition: '{}'", def)
            }
            AocError::PolicyDefinitionError(ref def) => {
                write!(f, "Invalid policy definition: '{}'", def)
            }
            AocError::UnknownBag(ref def) => write!(f, "Unknown bag: '{}'", def),
            AocError::InvalidInstruction(ref def) => write!(f, "Invalid instruction: '{}'", def),
            AocError::InvalidProgram(ref def) => write!(f, "Invalid program: '{}'", def),
            AocError::UnknownOperation(ref def) => write!(f, "Unknown operation: '{}'", def),
            AocError::IOError(ref err) => err.fmt(f),
            AocError::OsStringErr(ref err) => write!(f, "OsString error: {:?}", err),
            AocError::ParseIntError(ref err) => err.fmt(f),
            AocError::NoOutlier => write!(f, "No outlier found"),
            AocError::NoWeakness => write!(f, "No weakness found"),
            AocError::NoAdapterChain => write!(f, "No adapter chian found"),
            AocError::SeatDoesNotExist(ref pos) => write!(f, "Seast does not exist: '{:?}'", pos),
            AocError::GameError(ref msg) => write!(f, "Memory game error: '{}'", msg),
            AocError::NoValidRuleOrder => write!(f, "No rule order could be determined"),
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

impl From<std::ffi::OsString> for AocError {
    fn from(err: std::ffi::OsString) -> AocError {
        AocError::OsStringErr(err)
    }
}
