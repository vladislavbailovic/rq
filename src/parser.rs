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

    pub fn parse(&mut self) -> Result<FilterExpression, Error> {
        let mut filter: FilterExpression = Default::default();

        self.next()?;
        while self.token.is_some() {
            match &self.token {
                Some(Token::Bar) => {
                    let g: FilterGroup = Default::default();
                    filter.add_group(g);
                    let s: FilterSet = Default::default();
                    filter.add_set(s);
                }
                Some(Token::Comma) => {
                    filter.change_strategy(DataStrategy::Concat);
                    let s: FilterSet = Default::default();
                    filter.add_set(s);
                }
                Some(Token::Dot) => {
                    if let Some(t) = self.lex.peek()? {
                        match t {
                            Token::Word(word) => {
                                filter.add_filter(FilterType::Entry(word.to_string()));
                                self.lex.next()?;
                            }
                            _ => {
                                filter.add_filter(FilterType::Current);
                            }
                        }
                    }
                }
                Some(Token::OpenBracket) => {
                    filter.add_filter(self.parse_bracketed_expression()?);
                }
                Some(Token::Word(word)) => match word.as_str() {
                    "keys" => filter.add_filter(FilterType::Keys),
                    _ => return Err(Error::Parser(format!("unknown keyword: {}", word))),
                },
                _ => {
                    return Err(Error::Parser(format!(
                        "unexpected token: {}",
                        self.token.as_ref().unwrap()
                    )));
                }
            }
            self.next()?;
        }

        Ok(filter)
    }

    fn next(&mut self) -> Result<(), Error> {
        self.token = self.lex.next()?;
        Ok(())
    }

    fn new_range(start: usize, end: usize) -> Result<FilterType, Error> {
        return if (end > 0 && start >= end) || (start > 0 && start == end) {
            Err(Error::Parser(format!(
                "invalid range: start ({}) has to be less than end ({})",
                start, end
            )))
        } else {
            Ok(FilterType::Range(start, end))
        };
    }

    fn parse_bracketed_expression(&mut self) -> Result<FilterType, Error> {
        let mut token_set = Vec::new();
        let mut is_closed = false;
        while self.token.is_some() {
            self.next()?;
            if let Some(Token::CloseBracket) = &self.token {
                is_closed = true;
                break;
            }
            token_set.push(self.token.clone().unwrap());
        }
        if !is_closed {
            return Err(Error::Parser("bracketed expression not closed".to_string()));
        }
        return if token_set.is_empty() {
            // Empty array expression: []
            Ok(FilterType::Array)
        } else if token_set.contains(&Token::Colon) {
            // Range expression: [(n)?:(m)?]
            ExpressionParser::parse_range_expression(token_set)
        } else if token_set.len() == 1 {
            if let Token::Str(w) = &token_set[0] {
                // Object property: ["key"]
                return Ok(FilterType::Entry(w.to_string()));
            }
            if let Token::Number(n) = &token_set[0] {
                // Array member: [n]
                return Ok(FilterType::Member(n.parse::<usize>().unwrap()));
            }
            return Err(Error::Parser(format!(
                "expected number, string or range in bracketed expression, got {}",
                &token_set[0]
            )));
        } else {
            Err(Error::Parser("invalid bracket expression".to_string()))
        };
    }

    fn parse_range_expression(token_set: Vec<Token>) -> Result<FilterType, Error> {
        let mut start: usize = 0;
        let mut end: usize = 0;
        if token_set.len() == 1 {
            // Empty range expression: [:]
            return ExpressionParser::new_range(start, end);
        }

        let mut pos = 0;
        if let Token::Number(num) = &token_set[pos] {
            start = num.parse::<usize>().unwrap();
            pos += 1;
        }
        if let Token::Colon = &token_set[pos] {
            pos += 1;
        } else if pos == 0 {
            return Err(Error::Parser(format!(
                "expected colon or start range, got {}",
                &token_set[pos]
            )));
        } else {
            return Err(Error::Parser(
                "invalid range, expected colon separator".to_string(),
            ));
        }
        if pos < token_set.len() {
            if let Token::Number(num) = &token_set[pos] {
                end = num.parse::<usize>().unwrap();
            } else {
                return Err(Error::Parser(format!(
                    "expected end range, got {}",
                    &token_set[pos]
                )));
            }
        }
        ExpressionParser::new_range(start, end)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_keys_expr() {
        let mut parser = ExpressionParser::new(".[]keys");
        let result = parser.parse();

        assert!(result.is_ok(), "should be a success");

        // let filters = result.unwrap();
        // assert_eq!(3, filters.current_set().len(), "there should be 3 filters");
        // assert_eq!(
        //     FilterType::Current,
        //     filters.current_set()[0],
        //     "first filter type should be current"
        // );
        // assert_eq!(
        //     FilterType::Array,
        //     filters.current_set()[1],
        //     "second filter type should be array"
        // );
        // assert_eq!(
        //     FilterType::Keys,
        //     filters.current_set()[2],
        //     "last filter type should be keys"
        // );
    }

    #[test]
    fn parses_generic_object_index() {
        let mut parser = ExpressionParser::new("[\"what\"]");
        let result = parser.parse();

        assert!(result.is_ok(), "should be a success");

        // let filters = result.unwrap();
        // assert_eq!(1, filters.current_set().len(), "there should be 1 filter");
        // assert_eq!(
        //     FilterType::Entry("what".to_string()),
        //     filters.current_set()[0],
        //     "first filter type should be current"
        // );
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

        // let filters = result.unwrap();
        // assert_eq!(1, filters.current_set().len(), "there should be 1 filter");
        // assert_eq!(
        //     FilterType::Range(1, 61),
        //     filters.current_set()[0],
        //     "full range fully recognized"
        // );
    }

    #[test]
    fn parses_no_end_ranges() {
        let mut parser = ExpressionParser::new("[61:]");
        let result = parser.parse();

        assert!(result.is_ok(), "should not be an error");

        // let filters = result.unwrap();
        // assert_eq!(1, filters.current_set().len(), "there should be 1 filter");
        // assert_eq!(
        //     FilterType::Range(61, 0),
        //     filters.current_set()[0],
        //     "no end range fully recognized"
        // );
    }

    #[test]
    fn parses_no_start_ranges() {
        let mut parser = ExpressionParser::new("[:61]");
        let result = parser.parse();

        assert!(result.is_ok(), "should not be an error");

        // let filters = result.unwrap();
        // assert_eq!(1, filters.current_set().len(), "there should be 1 filter");
        // assert_eq!(
        //     FilterType::Range(0, 61),
        //     filters.current_set()[0],
        //     "no start range fully recognized"
        // );
    }

    #[test]
    fn parses_open_ranges() {
        let mut parser = ExpressionParser::new("[:]");
        let result = parser.parse();

        assert!(result.is_ok(), "should not be an error");

        // let filters = result.unwrap();
        // assert_eq!(1, filters.current_set().len(), "there should be 1 filter");
        // assert_eq!(
        //     FilterType::Range(0, 0),
        //     filters.current_set()[0],
        //     "open range fully recognized"
        // );
    }

    #[test]
    fn bar_adds_sets() {
        let mut parser = ExpressionParser::new("[]|keys");
        let result = parser.parse();
        assert!(result.is_ok(), "should not be an error");

        // let filters = result.unwrap();
        // assert_eq!(2, filters.len(), "there should be 2 filter sets");
    }
}
