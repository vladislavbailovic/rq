mod error;
use error::*;

mod filter;
use filter::*;

mod lexer;

mod parser;
use parser::*;

fn main() -> Result<(), Error> {
    let mut parser = ExpressionParser::new(".[0]|.name");
    let filters = parser.parse()?;
    println!("{:?}", filters);

    let filename = "test-data/one.json";
    let contents = std::fs::read_to_string(filename)?;
    let data = json::parse(&contents)?;

    let mut filter = Filter::new(filters);
    println!("{:?}", filter.apply(data)?);
    Ok(())
}
