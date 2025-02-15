
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

//NOTES:
//method has to be one of the predefined HTTP methods, so it can't simply be "a string" => use Enumeration (Enum)
//Enums have a finite set of possible values, and each value has a name associated with it
