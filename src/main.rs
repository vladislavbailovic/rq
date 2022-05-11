mod error;
use error::*;

mod filter;
use filter::*;

mod lexer;

mod parser;
use parser::*;

mod dataset;
use dataset::*;

fn main() -> Result<(), Error> {
    let filename = "test-data/one.yaml";
    let data = load_file(filename)?;

    println!("DATA IS:\n{:?}\nEND DATA", data);

    Ok(())
}
