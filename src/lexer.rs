use std::fmt::{Display, Formatter};
use std::iter::Peekable;

use crate::error::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    Number(String),
    Word(String),
    Str(String),
    Dot,
    Bar,
    Colon,
}

#[derive(Debug)]
pub struct Lexer<Chars: Iterator<Item = char>> {
    source: Peekable<Chars>,
    next_token: Option<Token>,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let kind = match &self {
            Token::OpenBracket => "open bracket".to_string(),
            Token::CloseBracket => "close bracket".to_string(),
            Token::Dot => "dot".to_string(),
            Token::Bar => "bar".to_string(),
            Token::Colon => "colon".to_string(),
            Token::Number(num) => format!("number {}", num),
            Token::Word(w) => format!("word {}", w),
            Token::Str(s) => format!("string {}", s),
        };
        write!(f, "{}", kind)
    }
}

impl<Chars: Iterator<Item = char>> Lexer<Chars> {
    fn create(chars: Chars) -> Self {
        Self {
            source: chars.peekable(),
            next_token: None,
        }
    }

    fn is_num(&self, c: char) -> bool {
        let c8 = c as u8;
        (48..=57).contains(&c8)
    }

    fn is_alpha(&self, c: char) -> bool {
        let c8 = c as u8;
        (65..=90).contains(&c8) // Uppercase
            || (97..=122).contains(&c8) // Lowercase
            || c8 == 95 // Underscore
    }

    fn is_alnum(&self, c: char) -> bool {
        self.is_num(c) || self.is_alpha(c)
    }

    pub fn next(&mut self) -> Result<Option<Token>, Error> {
        let mut next = self.next_token.clone();
        if next.is_none() {
            next = self.peek()?;
            if next.is_none() {
                return Ok(None);
            }
        }
        self.next_token = None;
        self.peek()?;
        Ok(next)
    }

    pub fn peek(&mut self) -> Result<Option<Token>, Error> {
        if self.next_token.is_some() {
            return Ok(self.next_token.clone());
        }
        self.next_token = self.get_next()?;
        Ok(self.next_token.clone())
    }

    pub fn get_next(&mut self) -> Result<Option<Token>, Error> {
        match self.source.next() {
            Some(c) => {
                match c {
                    '.' => Ok(Some(Token::Dot)),
                    '|' => Ok(Some(Token::Bar)),
                    ':' => Ok(Some(Token::Colon)),
                    '[' => Ok(Some(Token::OpenBracket)),
                    ']' => Ok(Some(Token::CloseBracket)),
                    '"' => {
                        let mut string = String::new();
                        for c in self.source.by_ref() {
                            if '"' == c {
                                return Ok(Some(Token::Str(string)));
                            }
                            string.push(c);
                        }
                        Err(Error::Lexer("Expected closing quote".to_string()))
                    }
                    _ => {
                        // Number
                        if self.is_num(c) {
                            let mut word = vec![c];
                            while let Some(&cis) = self.source.peek() {
                                if self.is_num(cis) {
                                    word.push(cis);
                                    self.source.next();
                                } else {
                                    break;
                                }
                            }
                            return Ok(Some(Token::Number(word.iter().collect())));
                        }

                        // Word
                        if self.is_alpha(c) {
                            let mut word = vec![c];
                            while let Some(&cis) = self.source.peek() {
                                if self.is_alnum(cis) {
                                    word.push(cis);
                                    self.source.next();
                                } else {
                                    break;
                                }
                            }
                            return Ok(Some(Token::Word(word.iter().collect())));
                        }

                        if !c.is_whitespace() {
                            return Err(Error::Lexer(format!("Unexpected char: {}", c)));
                        }

                        self.next()
                    }
                }
            }

            None => Ok(None),
        }
    }
}

impl Lexer<std::vec::IntoIter<char>> {
    pub fn new(expr: &str) -> Self {
        let chr = expr.chars().collect::<Vec<_>>().into_iter();
        Self::create(chr)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn recognizes_chars_alpha() {
        let lex = Lexer::new("");
        assert!(lex.is_alpha('t'), "t is alpha");
        assert!(!lex.is_alpha('1'), "1 is not alpha");
        assert!(!lex.is_alpha('#'), "# is not alpha");
    }

    #[test]
    fn recognizes_chars_num() {
        let lex = Lexer::new("");
        assert!(!lex.is_num('t'), "t is not num");
        assert!(lex.is_num('1'), "1 is num");
        assert!(!lex.is_num('#'), "# is not num");
    }

    #[test]
    fn recognizes_chars_alnum() {
        let lex = Lexer::new("");
        assert!(lex.is_alnum('t'), "t is alnum");
        assert!(lex.is_alnum('1'), "1 is alnum");
        assert!(!lex.is_alnum('#'), "# is not alnum");
    }

    #[test]
    fn lexes_words() {
        let mut lex = Lexer::new("two words");
        let r1 = lex.next().unwrap();
        if let Some(Token::Word(w)) = r1 {
            assert_eq!(w, "two");
        } else {
            assert!(false, "expected word token");
        }

        let r2 = lex.next().unwrap();
        if let Some(Token::Word(w)) = r2 {
            assert_eq!(w, "words");
        } else {
            assert!(false, "expected word token");
        }

        let r3 = lex.next().unwrap();
        assert!(r3.is_none(), "expected end of input");
    }

    #[test]
    fn lexes_strings() {
        let mut lex = Lexer::new("\"whatever the hell this is\"");

        let r1 = lex.next().unwrap();
        if let Some(Token::Str(s)) = r1 {
            assert_eq!(s, "whatever the hell this is");
        } else {
            assert!(false, "expected string token");
        }

        let r2 = lex.next().unwrap();
        assert!(r2.is_none(), "expected end of input");
    }

    #[test]
    fn lexes_numbers() {
        let mut lex = Lexer::new("1312 161");
        let r1 = lex.next().unwrap();
        if let Some(Token::Number(w)) = r1 {
            assert_eq!(w, "1312");
        } else {
            assert!(false, "expected number token");
        }

        let r2 = lex.next().unwrap();
        if let Some(Token::Number(w)) = r2 {
            assert_eq!(w, "161");
        } else {
            assert!(false, "expected number token");
        }

        let r3 = lex.next().unwrap();
        assert!(r3.is_none(), "expected end of input");
    }

    #[test]
    fn lexes_ranges() {
        let mut lex = Lexer::new("[161:1312]");

        let r1 = lex.next().unwrap();
        assert_eq!(Some(Token::OpenBracket), r1, "expected open bracket");

        let r2 = lex.next().unwrap();
        if let Some(Token::Number(w)) = r2 {
            assert_eq!(w, "161");
        } else {
            assert!(false, "expected number token");
        }

        let r3 = lex.next().unwrap();
        assert_eq!(Some(Token::Colon), r3, "expected colon");

        let r4 = lex.next().unwrap();
        if let Some(Token::Number(w)) = r4 {
            assert_eq!(w, "1312");
        } else {
            assert!(false, "expected number token");
        }

        let r5 = lex.next().unwrap();
        assert_eq!(Some(Token::CloseBracket), r5, "expected close bracket");

        let r6 = lex.next().unwrap();
        assert!(r6.is_none(), "expected end of input");
    }
}
