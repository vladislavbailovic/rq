use crate::error::*;
use crate::dataset::*;

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

    pub fn apply(&mut self, original_data: Data) -> Result<Data, Error> {
        let mut data = original_data;
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

    fn apply_current(&self, data: Data) -> Option<Data> {
        let f = self.current;
        match &self.types[f] {
            FilterType::Current => Some(data),

            FilterType::Array => {
                if let Data::Array(arr) = data {
                    Some(Data::Array(arr.clone()))
                } else {
                    None
                }
            }

            FilterType::Range(start, end) => {
                if let Data::Array(arr) = data {
                    let mut list: Vec<Data> = Vec::new();
                    let mut idx = 0;
                    let end_corr = if *end > 0 {
                        *end
                    } else {
                        arr.len()
                    };
                    while idx < arr.len() {
                        if idx <= *start && idx < end_corr {
                            list.push(arr[idx].clone());
                        }
                        idx += 1;
                    }
                    return Some(Data::Array(list));
                }

                None
            }

            FilterType::Keys => {
                if let Data::Array(arr) = data {
                    let mut list: Vec<Data> = Vec::new();
                    let mut idx = 0;
                    while idx < arr.len() {
                        list.push(Data::Integer(idx as i64));
                        idx += 1;
                    }
                    return Some(Data::Array(list));
                }

                if let Data::Hash(map) = data {
                    let mut keys: Vec<Data> = Vec::new();
                    for key in map.keys() {
                        keys.push(Data::String(key.to_string()));
                    }
                    return Some(Data::Array(keys));
                }

                None
            }

            FilterType::Member(idx) => {
                if let Data::Array(arr) = data {
                    if idx < &arr.len() {
                        Some(arr[*idx].clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            FilterType::Entry(name) => {
                if let Data::Hash(map) = data {
                    if map.contains_key(name) {
                        Some(map.get(name).unwrap().clone())
                    } else {
                        None
                    }
                } else if let Data::Array(arr) = data {
                    let mut new_data: Vec<Data> = Vec::new();
                    for val in arr {
                        // TODO: if val has key...
                        new_data.push(val.clone());
                    }
                    Some(Data::Array(new_data))
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
