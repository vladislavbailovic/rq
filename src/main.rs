mod error;
use error::*;

mod filter;
use filter::*;

mod lexer;
use lexer::*;

struct ExpressionParser {
    lex: Lexer<std::vec::IntoIter<char>>,
    token: Option<Token>,
}

impl ExpressionParser {
    pub fn new(source: &str) -> Self {
        let lex = Lexer::new(source);
        Self { lex, token: None }
    }

    fn next(&mut self) -> Result<(), Error> {
        self.token = self.lex.next()?;
        Ok(())
    }

    pub fn parse(&mut self) -> Result<Vec<FilterType>, Error> {
        let mut sequence = Vec::new();

        self.next()?;
        while self.token.is_some() {
            match &self.token {
                Some(Token::Bar) => {}
                Some(Token::Dot) => {
                    if let Some(t) = self.lex.peek()? {
                        match t {
                            Token::Word(word) => {
                                sequence.push(FilterType::Entry(word.to_string()));
                                self.lex.next()?;
                            }
                            _ => {
                                sequence.push(FilterType::Current);
                            }
                        }
                    }
                }
                Some(Token::OpenBracket) => {
                    self.token = self.lex.next()?;
                    if let Some(Token::CloseBracket) = &self.token {
                        sequence.push(FilterType::Array);
                    } else {
                        let old = self.token.clone();
                        self.next()?;
                        if let Some(Token::CloseBracket) = &self.token {
                            match old {
                                Some(Token::Number(num)) => {
                                    sequence
                                        .push(FilterType::Member(num.parse::<usize>().unwrap()));
                                }
                                Some(Token::Str(word)) => {
                                    sequence.push(FilterType::Entry(word.to_string()));
                                }
                                _ => {
                                    return Err(Error::Parser(
                                        "expected number, word or close bracket".to_string(),
                                    ));
                                }
                            }
                        } else {
                            return Err(Error::Parser("expected close bracket".to_string()));
                        }
                    }
                }
                Some(Token::Word(word)) => match word.as_str() {
                    "keys" => sequence.push(FilterType::Keys),
                    _ => return Err(Error::Parser(format!("unknown keyword: {}", word))),
                },
                _ => {
                    return Err(Error::Parser(format!(
                        "unexpected token: {:?}",
                        self.token.as_ref().unwrap()
                    )));
                }
            }
            self.next()?;
        }

        Ok(sequence)
    }
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_keys_expr() {
        let mut parser = ExpressionParser::new(".[]|keys");
        let result = parser.parse();

        assert!(result.is_ok(), "should be a success");

        let filters = result.unwrap();
        assert_eq!(3, filters.len(), "there should be 3 filters");
        assert_eq!(
            FilterType::Current,
            filters[0],
            "first filter type should be current"
        );
        assert_eq!(
            FilterType::Array,
            filters[1],
            "second filter type should be array"
        );
        assert_eq!(
            FilterType::Keys,
            filters[2],
            "last filter type should be keys"
        );
    }

    #[test]
    fn parses_generic_object_index() {
        let mut parser = ExpressionParser::new("[\"what\"]");
        let result = parser.parse();

        assert!(result.is_ok(), "should be a success");

        let filters = result.unwrap();
        assert_eq!(1, filters.len(), "there should be 1 filter");
        assert_eq!(
            FilterType::Entry("what".to_string()),
            filters[0],
            "first filter type should be current"
        );
    }

    #[test]
    fn expects_number_for_array_member() {
        let mut parser = ExpressionParser::new("[|]");
        let result = parser.parse();

        assert!(result.is_err(), "should not be a success");
    }

    #[test]
    fn expects_valid_tokens_sequence() {
        let mut parser = ExpressionParser::new("(what the...)");
        let result = parser.parse();

        assert!(result.is_err(), "should not be a success");
    }
}
