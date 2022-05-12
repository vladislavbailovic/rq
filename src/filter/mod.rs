use crate::dataset::*;
use crate::error::*;

mod r#type;
pub use r#type::*;

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
        self.types[f].apply(data)
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
