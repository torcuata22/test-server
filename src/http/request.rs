use super::method::{Method, MethodError};
use super::QueryString;
use core::str;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;
use std::string::String;

//in this case, the lifetime of the request is the lifetime of the buffer, so name it'buf
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

//because Request is generic, I need to give it a lifetime in the implementation, need to add it to impl block
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        //transform buffer into string slice
        let request = str::from_utf8(buf)?;
        //parse contents: first extract method, then path and query string, and lastl the protocol
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; //we are reassigning the var named "request" this is called variable shadowing
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; //we are reassigning the var named "request" this is called variable shadowing
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; //we are reassigning the var named "request" this is called variable shadowing

        //in this case, only suppor http 1.1, so add check for that:
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = method.parse()?;
        //separate query string fromthe path:
        let mut query_string: Option<QueryString<'buf>> = None;
        //need to find the "?" in the request -> "if let" allows pattern matching in an if statement
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }
        Ok(Self {
            path,
            query_string,
            method,
        })
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
    None::<(&str, &str)>;
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
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
//to convert method : &str into Enum: see method.rs

//one (clunky) way to separate the path and the query string:
// match path.find("?") {
//     Some(i) => {
//         query_string = Some(&path[i + 1..].to_string()); //everything after the '?' is the query string
//         path = &path[..i]; //path is everything before '?'
//     }
//     None => {}
// }

//to use lifetimes in a struct I need to make the struct generic over a lifetime (not a type)
//usually lifetimes are given a single letter name, for example < 'a>
