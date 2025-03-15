//specify public interface for the module, the first two lines (pub use...) are so I can import http::Method and http::Request in main.rs

pub use method::Method;
pub use query_string::{QueryString, Value as QueryStringValue};
pub use request::ParseError;
pub use request::Request;

pub mod method;
pub mod query_string;
pub mod request;
