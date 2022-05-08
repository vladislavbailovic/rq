mod error;
use error::*;

mod filter;
use filter::*;

mod lexer;
use lexer::*;

fn main() -> Result<(), Error> {
    let lex = Lexer::new("161 some tokens 1312 wat");
    for token in lex {
        println!("{:?}", token);
    }
    Ok(())
}

