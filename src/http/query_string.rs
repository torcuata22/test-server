//this file holds the query string struct and its functionality
use std::collections::HashMap;
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        //need to split the string in the &'s
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }
            data.entry(key)
                .and_modify(|existing| match existing {
                    Value::Single(prev_val) => {
                        let mut vec = vec![prev_val, val];
                        *existing = Value::Multiple(vec);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}

//HashMap has two type parameters:the type of the key and the type of the value
// need to link this file to the rest of the project (for the compiler), you do that in
//mod.rs and pull the query_string module
//need to add lifetime to the struct, too

//the query string hash needs to be able to handle more than one data type (some query strings
// might have more than one value for the same key, which would make them an array, for example)
//to handle this, we define the Enum
//for the array we would need to know the length at compile time,which we don't.
//To handle this, we need to use a vector, because these data types are heap allocated,
//and can grow dynamically

//also need a function to read the keys from the hashmap (impl)
