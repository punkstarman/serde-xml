use std::error;
use std::fmt::{self, Display};
//use std::io;
use std::num::ParseIntError;
use std::result;

use xml::reader;
//use xml::writer;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(Box<ErrorImpl>);

#[derive(Debug)]
pub enum ErrorImpl {
    Message(String),
    Reader(reader::Error),
    ParseIntError(ParseIntError),
}

pub fn with_message(s: String) -> Error {
    Error(Box::new(ErrorImpl::Message(s)))
}

pub fn parse_int(err: ParseIntError) -> Error {
    Error(Box::new(ErrorImpl::ParseIntError(err)))
}

pub fn reader(err: reader::Error) -> Error {
    Error(Box::new(ErrorImpl::Reader(err)))
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorImpl::Message(ref m) => write!(f, "{}", m),
            ErrorImpl::Reader(ref err) => write!(f, "{}", err),
            ErrorImpl::ParseIntError(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {
    
}

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error(Box::new(ErrorImpl::Message(msg.to_string())))
    }
}

impl serde::ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error(Box::new(ErrorImpl::Message(msg.to_string())))
    }
}