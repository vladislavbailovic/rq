use crate::dataset::*;
use crate::error::*;

mod r#type;
pub use r#type::*;

mod set;
pub use set::*;

mod group;
pub use group::*;

mod expression;
pub use expression::*;

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
