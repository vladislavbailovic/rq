use crate::error::*;

#[derive(Debug, PartialEq)]
pub enum FilterType {
    Current,
    Array,
    Keys,
    Member(usize),
    Entry(String),
    Range(usize, usize),
}

pub struct Filter {
    types: Vec<FilterType>,
    current: usize,
}

impl Filter {
    pub fn new(types: Vec<FilterType>) -> Self {
        Self { types, current: 0 }
    }

    pub fn apply(&mut self, original_data: json::JsonValue) -> Result<json::JsonValue, Error> {
        let mut data = original_data.clone();
        while self.current < self.types.len() {
            if let Some(new_data) = self.apply_current(data) {
                data = new_data.clone();
                self.current += 1;
            } else {
                return Err(Error::Filter);
            }
        }
        Ok(data)
    }

    fn apply_current(&self, data: json::JsonValue) -> Option<json::JsonValue> {
        let f = self.current;
        match &self.types[f] {
            FilterType::Current => Some(data),

            FilterType::Array => {
                if data.is_array() {
                    Some(data)
                } else {
                    None
                }
            }

            FilterType::Range(start, end) => {
                if data.is_array() {
                    let members = data
                        .members()
                        .enumerate()
                        .filter(|&(i, _)| i >= *start && (*end == 0 || (*end > 0 && i < *end)))
                        .map(|(_, x)| x)
                        .cloned();
                    let res: Vec<json::JsonValue> = members.collect();
                    Some(json::JsonValue::Array(res))
                } else {
                    None
                }
            }

            // FilterType::Next => {
            //     if data.is_array() {
            //         return if let Some(new_data) = data.members().next() {
            //             Some(new_data.clone())
            //         } else {
            //             None
            //         };
            //     }
            //     if data.is_object() {
            //         return if let Some(new_data) = data.entries().next() {
            //             Some(new_data.1.clone())
            //         } else {
            //             None
            //         };
            //     }
            //     None
            // }
            FilterType::Keys => {
                if data.is_array() {
                    let keys: Vec<usize> = (0..data.members().len()).collect();
                    return Some(json::JsonValue::from(keys));
                }
                if data.is_object() {
                    let keys: Vec<String> = data.entries().map(|x| x.0.to_string()).collect();
                    return Some(json::JsonValue::from(keys));
                }
                None
            }

            FilterType::Member(idx) => {
                if data.is_array() {
                    if let Some(new_data) = data.members().nth(*idx) {
                        Some(new_data.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            FilterType::Entry(name) => {
                if data.is_object() {
                    if data.has_key(name) {
                        Some(data[name].clone())
                    } else {
                        None
                    }
                } else if data.is_array() {
                    let new_data: Vec<json::JsonValue> = data
                        .members()
                        .filter(|x| x.has_key(name))
                        .map(|x| x[name].clone())
                        .collect();
                    return Some(json::JsonValue::from(new_data));
                } else {
                    None
                }
            }
        }
    }
}

// fn main() -> Result<(), Error> {
//     let filename = "test-data/one.json";
//     let contents = std::fs::read_to_string(filename)?;
//     let data = json::parse(&contents)?;
//     let mut filter = Filter::new(vec![
//         FilterType::Current,
//         FilterType::Array,
//         FilterType::Member(0),
//         FilterType::Entry("name".to_string())
//     ]);
//     println!("{:?}", filter.apply(data));
//     Ok(())
// }
