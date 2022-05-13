use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum FilterType {
    Current,
    Array,
    Keys,
    Member(usize),
    Entry(String),
    Range(usize, usize),
}

impl Filterable for FilterType {
    fn get_filterables(&self) -> Vec<Box<dyn Filterable>> {
        Vec::new()
    }
    fn apply(&self, data: Data) -> Result<Data, Error> {
        match &self {
            FilterType::Current => Ok(data),
            FilterType::Array => self.array(data),
            FilterType::Range(start, end) => self.range(data, *start, *end),
            FilterType::Keys => self.keys(data),
            FilterType::Member(idx) => self.member(data, *idx),
            FilterType::Entry(name) => self.entry(data, name.to_string()),
        }
    }
}

impl std::fmt::Display for FilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FilterType::Current => write!(f, "."),
            FilterType::Array => write!(f, "[]"),
            FilterType::Keys => write!(f, "keys"),
            FilterType::Member(n) => write!(f, "[{}]", n),
            FilterType::Entry(n) => write!(f, "[\"{}\"]", n),
            FilterType::Range(m, n) => write!(f, "[{}:{}]", m, n),
        }
    }
}

impl FilterType {
    fn array(&self, data: Data) -> Result<Data, Error> {
        if let Data::Array(arr) = data {
            Ok(Data::Array(arr))
        } else {
            Err(Error::Filter)
        }
    }

    fn keys(&self, data: Data) -> Result<Data, Error> {
        match data {
            Data::Array(arr) => {
                let mut list: Vec<Data> = Vec::new();
                let mut idx = 0;
                while idx < arr.len() {
                    list.push(Data::Integer(idx as i64));
                    idx += 1;
                }
                Ok(Data::Array(list))
            }
            Data::Hash(map) => {
                let mut keys: Vec<Data> = Vec::new();
                for key in map.keys() {
                    keys.push(Data::String(key.to_string()));
                }
                Ok(Data::Array(keys))
            }

            _ => Err(Error::Filter),
        }
    }

    fn member(&self, data: Data, idx: usize) -> Result<Data, Error> {
        if let Data::Array(arr) = data {
            if idx < arr.len() {
                Ok(arr[idx].clone())
            } else {
                Err(Error::Filter)
            }
        } else {
            Err(Error::Filter)
        }
    }

    fn entry(&self, data: Data, name: String) -> Result<Data, Error> {
        match data {
            Data::Hash(map) => {
                if map.contains_key(&name) {
                    Ok(map.get(&name).unwrap().clone())
                } else {
                    Err(Error::Filter)
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
                Ok(Data::Array(new_data))
            }
            _ => Err(Error::Filter),
        }
    }

    fn range(&self, data: Data, start: usize, end: usize) -> Result<Data, Error> {
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
            Ok(Data::Array(list))
        } else {
            Err(Error::Filter)
        }
    }
}
