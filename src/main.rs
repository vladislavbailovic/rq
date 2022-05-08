#[derive(Debug)]
enum Error {
    ParseError(String),
    FilterError,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        return Error::ParseError(format!("Unable to parse data because: {}", e));
    }
}

impl From<json::Error> for Error {
    fn from(e: json::Error) -> Self {
        return Error::ParseError(format!("Unable to parse data because: {}", e));
    }
}

enum FilterType {
    Current,
    Array,
    Next,
    Keys,
    Member(usize),
    Entry(String),
}

struct Filter {
    types: Vec<FilterType>,
    current: usize,
}

impl Filter {
    fn apply(&mut self, original_data: json::JsonValue) -> Result<json::JsonValue, Error> {
        let mut data = original_data.clone();
        while self.current < self.types.len() {
            if let Some(new_data) = self.apply_current(data) {
                data = new_data.clone();
                self.current += 1;
            } else {
                return Err(Error::FilterError);
            }
        }
        Ok(data)
    }

    fn apply_current(&self, data: json::JsonValue) -> Option<json::JsonValue> {
        let f = self.current;
        match &self.types[f] {
            FilterType::Current => Some(data),

            FilterType::Array => if data.is_array() {
                Some(data)
            } else {
                None
            }

            FilterType::Next => {
                if data.is_array() {
                    return if let Some(new_data) = data.members().next() {
                        Some(new_data.clone())
                    } else {
                        None
                    };
                }
                if data.is_object() {
                    return if let Some(new_data) = data.entries().next() {
                        Some(new_data.1.clone())
                    } else {
                        None
                    };
                }
                None
            }

            FilterType::Keys => {
                if data.is_array() {
                    let keys: Vec<usize> = (0..data.members().len()).collect();
                    return Some(json::JsonValue::from(keys));
                }
                if data.is_object() {
                    let keys: Vec<String> = data.entries().map(|x|x.0.to_string()).collect();
                    return Some(json::JsonValue::from(keys));
                }
                None
            }

            FilterType::Member(idx) => if data.is_array() {
                if let Some(new_data) = data.members().nth(*idx) {
                    Some(new_data.clone())
                } else {
                    None
                }
            } else {
                None
            }

            FilterType::Entry(name) => if data.is_object() {
                if data.has_key(name) {
                    Some(data[name].clone())
                } else {
                    Some(json::JsonValue::from("no such key"))
                }
            } else {
                None
            }

            _ => Some(json::JsonValue::from("wat")),
        }
    }
}

fn main() -> Result<(), Error> {
    let filename = "test-data/one.json";
    let contents = std::fs::read_to_string(filename)?;
    let data = json::parse(&contents)?;
    let mut filter = Filter {
        types: vec![FilterType::Current, FilterType::Array, FilterType::Member(0), FilterType::Entry("name".to_string())],
        current: 0,
    };
    println!("{:?}", filter.apply(data));
    Ok(())
}
