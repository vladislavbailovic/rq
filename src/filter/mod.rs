use crate::dataset::*;
use crate::error::*;

mod r#type;
pub use r#type::*;

mod set;
pub use set::*;

pub trait Filterable {
    fn get_filterables(&self) -> Vec<Box<dyn Filterable>>;

    fn apply(&self, original_data: Data) -> Result<Data, Error> {
        let mut data = original_data;
        let filterables = self.get_filterables();
        for filterable in filterables {
            let new_data = filterable.apply(data)?;
            data = new_data.clone();
        }
        Ok(data)
    }
}

#[derive(Debug, Default)]
pub struct Filter {
    // TODO: only pub because of parser tests, address this!
    pub sets: Vec<FilterSet>,
}

impl Filter {
    pub fn add_set(&mut self, s: FilterSet) {
        self.sets.push(s);
    }

    pub fn add(&mut self, t: FilterType) {
        if self.sets.is_empty() {
            let s: FilterSet = Default::default();
            self.add_set(s);
        }
        let l = self.sets.len();
        self.sets[l - 1].add(t);
    }
}

impl Filterable for Filter {
    fn get_filterables(&self) -> Vec<Box<dyn Filterable>> {
        let mut map: Vec<Box<dyn Filterable>> = Vec::new();
        for set in &self.sets {
            map.push(Box::new(set.clone()));
        }
        map
    }
}
