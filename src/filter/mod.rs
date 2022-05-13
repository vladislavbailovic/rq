use crate::dataset::*;
use crate::error::*;

mod r#type;
pub use r#type::*;

mod set;
pub use set::*;

mod group;
pub use group::*;

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
    groups: Vec<FilterGroup>,
}

impl Filter {

    pub fn add_set(&mut self, s: FilterSet) {
        let l = self.groups.len();
        self.groups[l - 1].add(s);
    }

    pub fn add(&mut self, t: FilterType) {
        if self.groups.is_empty() {
            let g: FilterGroup = Default::default();
            self.groups.push(g);
        }
        let l = self.groups.len();
        self.groups[l - 1].add_filter(t);
    }
}

impl Filterable for Filter {
    fn get_filterables(&self) -> Vec<Box<dyn Filterable>> {
        let mut map: Vec<Box<dyn Filterable>> = Vec::new();
        for set in &self.groups {
            map.push(Box::new(set.clone()));
        }
        map
    }
}
