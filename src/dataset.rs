use crate::error::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Data {
    Hash(HashMap<String, Data>),
    Array(Vec<Data>),
    String(String),
    Integer(i64),
    Real(f32),
    Boolean(bool),
}

pub fn load_file(filename: &str) -> Result<Data, Error> {
    let fpath = std::path::Path::new(filename);
    match fpath.extension() {
        None => Err(Error::Dataset("missing file extension".to_string())),
        Some(os_str) => match os_str.to_str() {
            None => Err(Error::Dataset("missing file extension".to_string())),
            Some("yaml") => load_yaml(filename),
            Some("yml") => load_yaml(filename),
            Some("json") => load_json(filename),
            Some(ext) => Err(Error::Dataset(format!("unknown file extension: {}", ext))),
        },
    }
}

fn load_yaml(filename: &str) -> Result<Data, Error> {
    let contents = std::fs::read_to_string(filename)?;
    let raw = yaml_rust::YamlLoader::load_from_str(&contents)?;
    return if raw.len() == 1 {
        parse_yaml(raw[0].clone())
    } else {
        let mut arr: Vec<Data> = Vec::new();
        for item in raw {
            arr.push(parse_yaml(item)?);
        }
        Ok(Data::Array(arr))
    };
}

fn parse_yaml(raw: yaml_rust::yaml::Yaml) -> Result<Data, Error> {
    use yaml_rust::yaml::Yaml;
    match raw {
        Yaml::Real(string) => Ok(Data::Real(string.parse::<f32>().unwrap())),
        Yaml::Integer(num) => Ok(Data::Integer(num)),
        Yaml::String(string) => Ok(Data::String(string)),
        Yaml::Hash(map) => {
            let mut hash: HashMap<String, Data> = HashMap::new();
            for (key, value) in map {
                hash.insert(key.as_str().unwrap().to_string(), parse_yaml(value)?);
            }
            Ok(Data::Hash(hash))
        }
        Yaml::Array(list) => {
            let mut arr: Vec<Data> = Vec::new();
            for el in list {
                arr.push(parse_yaml(el)?);
            }
            Ok(Data::Array(arr))
        }
        Yaml::Boolean(b) => Ok(Data::Boolean(b)),
        _ => todo!("Not covered"),
    }
}

fn load_json(filename: &str) -> Result<Data, Error> {
    let contents = std::fs::read_to_string(filename)?;
    let raw = json::parse(&contents)?;
    parse_json(raw)
}

fn parse_json(raw: json::JsonValue) -> Result<Data, Error> {
    return if raw.is_array() {
        let mut arr: Vec<Data> = Vec::new();
        for item in raw.members() {
            arr.push(parse_json(item.clone())?);
        }
        Ok(Data::Array(arr))
    } else if raw.is_object() {
        let mut hash: HashMap<String, Data> = HashMap::new();
        for (key, value) in raw.entries() {
            hash.insert(key.to_string(), parse_json(value.clone())?);
        }
        Ok(Data::Hash(hash))
    } else if raw.is_number() {
        let num = raw.as_i64();
        if num.is_none() {
            let num = raw.as_f32();
            if num.is_some() {
                Ok(Data::Real(num.unwrap() as f32))
            } else {
                Err(Error::Dataset(format!("invalid number: {:?}", num)))
            }
        } else {
            Ok(Data::Integer(num.unwrap()))
        }
    } else if raw.is_boolean() {
        Ok(Data::Boolean(raw.as_bool().unwrap()))
    } else if raw.is_string() {
        Ok(Data::String(raw.as_str().unwrap().to_string()))
    } else {
        Err(Error::Dataset(
            "JSON is not an object or an array".to_string(),
        ))
    };
}
