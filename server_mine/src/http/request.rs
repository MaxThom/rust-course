
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::{str, Utf8Error};
use std::fmt::{Formatter, Display, Result as FmtResult, Debug};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

//impl Request {
//    fn from_byte_array(buff: &[u8]) -> Result<Self, String> {
//
//    }
//}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' {
            return Some((&request[..i], &request[i+1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError { }

//trait Encrypt {
//    fn encrypt(&self) -> Self;
//}
//
//impl Encrypt for String {
//    fn encrypt(&self) -> Self {
//        unimplemented!()
//    }
//}
//
//impl Encrypt for &[u8] {
//    fn encrypt(&self) -> Self {
//        unimplemented!()
//    }
//}