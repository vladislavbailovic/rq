use crate::error::*;
use crate::filter::*;
use crate::lexer::*;

pub struct ExpressionParser {
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

    fn new_range(start: usize, end: usize) -> Result<FilterType, Error> {
        return if (end > 0 && start >= end) || (start == end) {
            Err(Error::Parser(format!(
                "invalid range: start ({}) has to be less than end ({})",
                start, end
            )))
        } else {
            Ok(FilterType::Range(start, end))
        };
    }

    fn parse_bracketed_expression(&mut self) -> Result<FilterType, Error> {
        #[allow(unused_assignments)]
        let mut start: usize = 0;
        let mut end: usize = 0;
        let old = self.token.clone();
        self.next()?;
        if let Some(Token::CloseBracket) = &self.token {
            match old {
                Some(Token::Number(num)) => {
                    return Ok(FilterType::Member(num.parse::<usize>().unwrap()));
                }
                Some(Token::Str(word)) => {
                    return Ok(FilterType::Entry(word.to_string()));
                }
                Some(t) => {
                    return Err(Error::Parser(format!(
                        "expected number, word or close bracket, got {}",
                        t
                    )));
                }
                _ => {
                    return Err(Error::Parser(
                        "expected number, word or close bracket, got nothing".to_string(),
                    ));
                }
            }
        } else if let Some(Token::Colon) = &self.token {
            match old {
                Some(Token::Number(num)) => {
                    start = num.parse::<usize>().unwrap();
                }
                _ => {
                    return Err(Error::Parser("expected number for range start".to_string()));
                }
            }
            self.next()?;
            if let Some(Token::CloseBracket) = &self.token {
                // End size omitted
                return ExpressionParser::new_range(start, end);
            } else if let Some(Token::Number(num)) = &self.token {
                end = num.parse::<usize>().unwrap();
                self.next()?;
                if let Some(Token::CloseBracket) = &self.token {
                    // Both start and end sizes given
                    return ExpressionParser::new_range(start, end);
                } else {
                    return Err(Error::Parser(
                        "expected close bracket after range".to_string(),
                    ));
                }
            } else {
                return Err(Error::Parser(
                    "expected range end or close bracket".to_string(),
                ));
            }
        } else if let Some(Token::Number(num)) = &self.token {
            if let Some(Token::Colon) = old {
                // No start range
                end = num.parse::<usize>().unwrap();
                self.next()?;
                if let Some(Token::CloseBracket) = &self.token {
                    return ExpressionParser::new_range(start, end);
                } else {
                    return Err(Error::Parser(
                        "expected close bracket after range".to_string(),
                    ));
                }
            } else {
                return Err(Error::Parser("unable to parse range".to_string()));
            }
        } else if let Some(t) = &self.token {
            return Err(Error::Parser(format!(
                "expected close bracket or colon, got {}",
                t
            )));
        } else {
            return Err(Error::Parser(
                "expected close bracket or colon, got nothing".to_string(),
            ));
        }
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
                        sequence.push(self.parse_bracketed_expression()?);
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

    #[test]
    fn parses_full_ranges() {
        let mut parser = ExpressionParser::new("[1:61]");
        let result = parser.parse();

        assert!(result.is_ok(), "should not be an error");

        let filters = result.unwrap();
        assert_eq!(1, filters.len(), "there should be 1 filter");
        assert_eq!(
            FilterType::Range(1, 61),
            filters[0],
            "full range fully recognized"
        );
    }

    #[test]
    fn parses_no_end_ranges() {
        let mut parser = ExpressionParser::new("[61:]");
        let result = parser.parse();

        assert!(result.is_ok(), "should not be an error");

        let filters = result.unwrap();
        assert_eq!(1, filters.len(), "there should be 1 filter");
        assert_eq!(
            FilterType::Range(61, 0),
            filters[0],
            "no end range fully recognized"
        );
    }

    #[test]
    fn parses_no_start_ranges() {
        let mut parser = ExpressionParser::new("[:61]");
        let result = parser.parse();

        assert!(result.is_ok(), "should not be an error");

        let filters = result.unwrap();
        assert_eq!(1, filters.len(), "there should be 1 filter");
        assert_eq!(
            FilterType::Range(0, 61),
            filters[0],
            "no start range fully recognized"
        );
    }

    #[test]
    fn errors_out_on_open_ranges() {
        let mut parser = ExpressionParser::new("[:]");
        let result = parser.parse();

        assert!(result.is_err(), "should not be an error");
    }
}

// fn main() -> Result<(), Error> {
//     let mut parser = ExpressionParser::new(".[0]|.name");
//     let filters = parser.parse()?;
//     println!("{:?}", filters);

//     let filename = "test-data/one.json";
//     let contents = std::fs::read_to_string(filename)?;
//     let data = json::parse(&contents)?;

//     let mut filter = Filter::new(filters);
//     println!("{:?}", filter.apply(data)?);
//     Ok(())
// }
