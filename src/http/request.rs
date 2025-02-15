use crate::http::method::Method;
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

//NOTES:
//Create Struct for the request
//What if there is no query string? Rust doesn't support null values, to express the absence of a value, it uses Option<T>
//in this case, we can use Option<String> meaning we ar eusing an Option type that holds a String (value can be a string (present) or None (absent))

/*
GET /user?id=10 HTTP 1.1\r\n (focus on this)
HEADERS\r\n\ (ignore for now)
BODY (ignore for now)
*/
