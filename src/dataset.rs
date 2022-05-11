use crate::error::*;
use std::collections::HashMap;

#[derive(Debug)]
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
        None => Err(Error::Datasource("missing file extension".to_string())),
        Some(os_str) => match os_str.to_str() {
            None => Err(Error::Datasource("missing file extension".to_string())),
            Some("yaml") => load_yaml(filename),
            Some("yml") => load_yaml(filename),
            Some("json") => load_json(filename),
            Some(ext) => Err(Error::Datasource(format!("unknown file extension: {}", ext))),
        }
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
    }
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
        },
        Yaml::Array(list) => {
            let mut arr: Vec<Data> = Vec::new();
            for el in list {
                arr.push(parse_yaml(el)?);
            }
            Ok(Data::Array(arr))
        },
        Yaml::Boolean(b) => Ok(Data::Boolean(b)),
        _ => todo!("Not covered"),
    }
}

fn load_json(filename: &str) -> Result<Data, Error> {
    Err(Error::Datasource("nope".to_string()))
}
