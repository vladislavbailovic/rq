mod error;
use error::*;

mod filter;
use filter::*;

mod lexer;
use lexer::*;

use std::iter::Peekable;

struct ExpressionParser {
    lex: Peekable<Lexer<std::vec::IntoIter<char>>>,
    token: Option<Token>
}

impl ExpressionParser {
    pub fn new(source: &str) -> Self {
        let lex = Lexer::new(source).peekable();
        Self{ lex, token: None }
    }

    fn next(&mut self) {
        self.token = self.lex.next();
    }

    pub fn parse(&mut self) -> Result<Vec<FilterType>, Error> {
        let mut sequence= Vec::new();

        self.next();
        while self.token.is_some() {
            match &self.token {
                Some(Token::Dot) => {
                    if let Some(t) = self.lex.peek() {
                        match t {
                            Token::Word(word) => {
                                sequence.push(FilterType::Entry(word.to_string()));
                                self.lex.next();
                            }
                            _ => {
                                sequence.push(FilterType::Current);
                                self.lex.next();
                            }
                        }
                    }
                }
                Some(Token::OpenBracket) => {
                    self.token = self.lex.next();
                    if let Some(Token::CloseBracket) = &self.token {
                        sequence.push(FilterType::Array);
                    } else if let Some(Token::Number(num)) = self.token.clone() {
                        self.next();
                        if let Some(Token::CloseBracket) = &self.token {
                            sequence.push(FilterType::Member(num.parse::<usize>().unwrap()));
                        } else {
                            return Err(Error::ParseExpression("expected close bracket after number".to_string()));
                        }
                    } else if let Some(Token::Word(word)) = self.token.clone() {
                        self.next();
                        if let Some(Token::CloseBracket) = &self.token {
                            sequence.push(FilterType::Entry(word.to_string()));
                        } else {
                            return Err(Error::ParseExpression("expected close bracket after word".to_string()));
                        }
                    } else {
                        return Err(Error::ParseExpression("expected number, word or close bracket".to_string()));
                    }
                }
                Some(Token::Word(word)) => {
                    match word.as_str() {
                        "keys" => sequence.push(FilterType::Keys),
                        _ => return Err(Error::ParseExpression(format!("unknown keyword: {}", word))),
                    }
                }
                _ => {}
            }
            self.next();
        }

        Ok(sequence)
    }
}

fn main() -> Result<(), Error> {
    let mut parser = ExpressionParser::new(".[]|keys");
    let filters = parser.parse()?;

    let filename = "test-data/one.json";
    let contents = std::fs::read_to_string(filename)?;
    let data = json::parse(&contents)?;

    let mut filter = Filter::new(filters);
    println!("{:?}", filter.apply(data));
    Ok(())
}

