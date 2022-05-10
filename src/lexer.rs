use std::iter::Peekable;

use crate::error::*;

#[derive(Debug,Clone)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    Number(String),
    Word(String),
    Str(String),
    Dot,
    Bar,
}

#[derive(Debug)]
pub struct Lexer<Chars: Iterator<Item=char>> {
    source: Peekable<Chars>,
    next_token: Option<Token>
}

impl<Chars: Iterator<Item=char>> Lexer<Chars> {
    fn create(chars: Chars) -> Self {
        Self {
            source: chars.peekable(),
            next_token: None
        }
    }

    fn is_num(&self, c: char) -> bool {
        let c8 = c as u8;
        return c8 >= 48 && c8 <= 57;
    }

    fn is_alpha(&self, c: char) -> bool {
        let c8 = c as u8;
        return (c8 >= 65 && c8 <= 90) // Uppercase
            || (c8 >= 97 && c8 <= 122) // Lowercase
            || c8 == 95; // Underscore
    }

    fn is_alnum(&self, c: char) -> bool {
        return self.is_num(c) || self.is_alpha(c);
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
                    '[' => Ok(Some(Token::OpenBracket)),
                    ']' => Ok(Some(Token::CloseBracket)),
                    '"' => {
                        let mut string = String::new();
                        while let Some(c) = self.source.next() {
                            if '"' == c {
                                return Ok(Some(Token::Str(string)));
                            }
                            string.push(c);
                        }
                        Err(Error::LexError("Expected closing quote".to_string()))
                    }
                    _ => {

                        // Number
                        if self.is_num(c) {
                            let mut word = Vec::new();
                            word.push(c);
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
                            let mut word = Vec::new();
                            word.push(c);
                            while let Some(&cis) = self.source.peek() {
                                if self.is_alnum(cis) {
                                    word.push(cis);
                                    self.source.next();
                                } else {
                                    break
                                }
                            }
                            return Ok(Some(Token::Word(word.iter().collect())));
                        }

                        if !c.is_whitespace() {
                            return Err(Error::LexError(
                                format!("Unexpected char: {}", c)
                            ));
                        }

                        self.next()
                    }
                }
            }

            None => Ok(None)
        }
    }
}

impl Lexer<std::vec::IntoIter<char>> {
    pub fn new(expr: &str) -> Self {
        let chr = expr.chars().collect::<Vec<_>>().into_iter();
        Self::create(chr)
    }
}
