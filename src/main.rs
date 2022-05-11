mod error;
use error::*;

mod filter;
use filter::*;

mod lexer;

mod parser;
use parser::*;

mod dataset;
use dataset::*;

fn get_input(prompt: &str) -> Result<String, Error> {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{} ", prompt);
    stdout().flush()?;
    stdin().read_line(&mut s)?;

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    Ok(s)
}

fn report_error<T>(result: Result<T, Error>) {
    match result.err().unwrap() {
        Error::Lexer(err) => eprintln!("[Lexer] ERROR: {}", err),
        Error::Parser(err) => eprintln!("[Parser] ERROR: {}", err),
        Error::Filter => eprintln!("Expression did not match anything"),
        _ => eprintln!("Generic error"),
    }
}

fn main() -> Result<(), Error> {
    let filename = "test-data/one.json";
    let data = load_file(filename)?;

    loop {
        let input = get_input(">")?;
        if input == "q" || input == "quit" {
            break;
        }
        let mut parser = ExpressionParser::new(&input);
        let filters = parser.parse();
        if filters.is_err() {
            report_error(filters);
            continue;
        }
        println!("{:?}", filters);

        let mut filter = Filter::new(filters.unwrap());
        let result = filter.apply(data.clone());
        if result.is_err() {
            report_error(result);
            continue;
        }
        println!("{:?}", result.unwrap());
    }
    Ok(())
}
