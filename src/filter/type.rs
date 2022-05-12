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

impl FilterType {
    pub fn apply(&self, data: Data) -> Option<Data> {
        match &self {
            FilterType::Current => Some(data),
            FilterType::Array => self.array(data),
            FilterType::Range(start, end) => self.range(data, *start, *end),
            FilterType::Keys => self.keys(data),
            FilterType::Member(idx) => self.member(data, *idx),
            FilterType::Entry(name) => self.entry(data, name.to_string()),
        }
    }

    fn array(&self, data: Data) -> Option<Data> {
        if let Data::Array(arr) = data {
            Some(Data::Array(arr))
        } else {
            None
        }
    }

    fn keys(&self, data: Data) -> Option<Data> {
        match data {
            Data::Array(arr) => {
                let mut list: Vec<Data> = Vec::new();
                let mut idx = 0;
                while idx < arr.len() {
                    list.push(Data::Integer(idx as i64));
                    idx += 1;
                }
                Some(Data::Array(list))
            }
            Data::Hash(map) => {
                let mut keys: Vec<Data> = Vec::new();
                for key in map.keys() {
                    keys.push(Data::String(key.to_string()));
                }
                Some(Data::Array(keys))
            }

            _ => None,
        }
    }

    fn member(&self, data: Data, idx: usize) -> Option<Data> {
        if let Data::Array(arr) = data {
            if idx < arr.len() {
                Some(arr[idx].clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn entry(&self, data: Data, name: String) -> Option<Data> {
        match data {
            Data::Hash(map) => {
                if map.contains_key(&name) {
                    Some(map.get(&name).unwrap().clone())
                } else {
                    None
                }
            }
            Data::Array(arr) => {
                let mut new_data: Vec<Data> = Vec::new();
                for val in arr {
                    if let Data::Hash(obj) = val {
                        if obj.contains_key(&name) {
                            new_data.push(obj.get(&name).unwrap().clone());
                        }
                    }
                }
                Some(Data::Array(new_data))
            }
            _ => None,
        }
    }

    fn range(&self, data: Data, start: usize, end: usize) -> Option<Data> {
        if let Data::Array(arr) = data {
            let mut list: Vec<Data> = Vec::new();
            let mut idx = 0;
            let end_corr = if end > 0 { end } else { arr.len() };
            while idx < arr.len() {
                if idx >= start && idx < end_corr {
                    list.push(arr[idx].clone());
                }
                idx += 1;
            }
            Some(Data::Array(list))
        } else {
            None
        }
    }
}
