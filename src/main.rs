mod error;
use error::*;

mod filter;
use filter::*;

mod lexer;
use lexer::*;

fn parse(source: &str) -> Result<Vec<FilterType>, Error> {
    let mut lex = Lexer::new(source).peekable();
    let mut sequence= Vec::new();

    let mut token = lex.next();
    while token.is_some() {
        match token {
            Some(Token::Dot) => {
                if let Some(t) = lex.peek() {
                    match t {
                        Token::Word(word) => {
                            sequence.push(FilterType::Entry(word.to_string()));
                            lex.next();
                        }
                        _ => {}
                    }
                }
            }
            Some(Token::OpenBracket) => {
                token = lex.next();
                if let Some(Token::CloseBracket) = token {
                    sequence.push(FilterType::Array);
                } else if let Some(Token::Number(num)) = token {
                    token = lex.next();
                    if let Some(Token::CloseBracket) = token {
                        sequence.push(FilterType::Member(num.parse::<usize>().unwrap()));
                    } else {
                        return Err(Error::ParseExpression("expected close bracket after number".to_string()));
                    }
                } else if let Some(Token::Word(word)) = token {
                    token = lex.next();
                    if let Some(Token::CloseBracket) = token {
                        sequence.push(FilterType::Entry(word));
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
        token = lex.next();
    }

    Ok(sequence)
}

fn main() -> Result<(), Error> {
    let filters = parse(".[]|keys")?;

    let filename = "test-data/one.json";
    let contents = std::fs::read_to_string(filename)?;
    let data = json::parse(&contents)?;

    let mut filter = Filter::new(filters);
    println!("{:?}", filter.apply(data));
    Ok(())
}

