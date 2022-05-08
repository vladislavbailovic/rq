mod error;
use error::*;

mod filter;
use filter::*;

fn main() -> Result<(), Error> {
    let filename = "test-data/one.json";
    let contents = std::fs::read_to_string(filename)?;
    let data = json::parse(&contents)?;
    let mut filter = Filter::new(vec![
        FilterType::Current,
        FilterType::Array,
        FilterType::Member(0),
        FilterType::Entry("name".to_string())
    ]);
    println!("{:?}", filter.apply(data));
    Ok(())
}
