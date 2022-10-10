use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'value> {
    data: HashMap<&'value str, Value<'value>>,
}

#[derive(Debug)]
pub enum Value<'value> {
    Single(&'value str),
    Multiple(Vec<&'value str>),
}

impl<'value> QueryString<'value> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'value> From<&'value str> for QueryString<'value> {
    fn from(s: &'value str) -> Self {
        let mut data = HashMap::new();
        for substr in s.split('&') {
            let mut key = substr;
            let mut val = "";
            if let Some(i) = substr.find("=") {
                key = &substr[..i];
                val = &substr[(i + 1)..];
            }

            data.entry(key).and_modify(|existing| match existing {
                Value::Single(previous) => {
                    *existing = Value::Multiple(vec![previous, val]);
                },
                Value::Multiple(vec) => {
                    vec.push(val);
                },
            }).or_insert(Value::Single(val));
        }
        QueryString { data }
    }
}