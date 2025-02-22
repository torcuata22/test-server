use crate::http::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult}; //need it for error

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
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
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Error for ParseError {}

//NOTES:
//Create Struct for the request
//What if there is no query string? Rust doesn't support null values, to express the absence of a value, it uses Option<T>
//in this case, we can use Option<String> meaning we ar eusing an Option type that holds a String (value can be a string (present) or None (absent))

/*
GET /user?id=10 HTTP 1.1\r\n (focus on this)
HEADERS\r\n\ (ignore for now)
BODY (ignore for now)
*/

//fn from_byte_array(buf: &[u8]) -> Result<Self> returns a Result which, if successful, will contain
//a Request struct and if fails it will contain the error (string) especifying what part of the request failed
//use from module in std library to convert byte array into request
//unimplemented!();  can be used in any fn we are not ready to implement yet
