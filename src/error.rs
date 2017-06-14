use iron::error::IronError;
use iron::status;
use std;
use std::env::VarError;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum EmpholiteError {
    Custom(String),
    Env(VarError),
    IO(io::Error),
}

pub fn make_iron_error(message: &str, status: status::Status) -> IronError {
    IronError::new(EmpholiteError::Custom(message.into()), status)
}

macro_rules! error_wrap {
    ($f: ty, $e: expr) => {
        impl From<$f> for EmpholiteError {
            fn from(f: $f) -> EmpholiteError { $e(f) }
        }
    }
}

impl std::error::Error for EmpholiteError {
    fn description(&self) -> &str {
        match *self {
            EmpholiteError::Custom(ref message) => &message,
            EmpholiteError::Env(ref err) => err.description(),
            EmpholiteError::IO(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EmpholiteError::Custom(_) => None,
            EmpholiteError::Env(ref err) => Some(err),
            EmpholiteError::IO(ref err) => Some(err),
        }
    }
}

impl fmt::Display for EmpholiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EmpholiteError::Custom(ref message) => f.write_str(&message),
            EmpholiteError::Env(ref err) => err.fmt(f),
            EmpholiteError::IO(ref err) => err.fmt(f),
        }
    }
}

error_wrap!(io::Error, EmpholiteError::IO);
error_wrap!(VarError, EmpholiteError::Env);
