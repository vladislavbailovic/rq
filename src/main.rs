mod error;
use error::*;

mod filter;
use filter::*;

mod lexer;

mod parser;
use parser::*;

fn get_input(prompt: &str) -> Result<String, Error> {
    use std::io::{stdin,stdout,Write};
    let mut s = String::new();
    print!("{} ", prompt);
    stdout().flush()?;
    stdin().read_line(&mut s)?;

    if let Some('\n') = s.chars().next_back() { s.pop(); }
    if let Some('\r') = s.chars().next_back() { s.pop(); }

    Ok(s)
}

fn main() -> Result<(), Error> {
    let filename = "test-data/one.json";
    let contents = std::fs::read_to_string(filename)?;
    let data = json::parse(&contents)?;

    loop {
        let input = get_input(">")?;
        if input == "q" || input == "quit" {
            break;
        }
        let mut parser = ExpressionParser::new(&input);
        let filters = parser.parse()?;
        println!("{:?}", filters);

        let mut filter = Filter::new(filters);
        println!("{:?}", filter.apply(data.clone())?);
    }
    Ok(())
}
