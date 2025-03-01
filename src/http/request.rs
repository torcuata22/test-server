use crate::http::method::Method;
use core::str;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
// use std::str;
use std::str::Utf8Error;
use std::string::String;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        //transform buffer into string slice
        let request = str::from_utf8(buf)?;
        //parse contents: first extract method, then path and query string, and lastl the protocol
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; //we are reassigning the var named "request" this is called variable shadowing
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; //we are reassigning the var named "request" this is called variable shadowing
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; //we are reassigning the var named "request" this is called variable shadowing
        
        //in this case, only suppor http 1.1, so add check for that:
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        unimplemented!();
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    //lop through string until I find a space
    let mut iter = request.chars();
    //for loop, but I need to get the index of c, also, so use enumerate after chars, which returns tuple (i, val)
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
    unimplemented!();
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

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
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
//fn get_next_word(s: &str) -> Option<(&str, &str)> The tuple serves to parse strings
//first element corresponds to the word and the second element corresponds to the rest of the string
//when you run the functin again, pass the rest of the string as the argument and it will take the first
//word and the rest of the string, and so on
//wrap in Option to deal with cases where there is no next word

//to iterate over string I could:
// loop {
//     let item = iter.next();
//     match item {
//         Some(c) => r {},
//         None => break,
//     }
// }
