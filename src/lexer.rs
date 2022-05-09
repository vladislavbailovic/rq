use std::iter::Peekable;

#[derive(Debug,Clone)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    Number(String),
    Word(String),
    Str(String),
    Dot,
    Bar,
    Any(char),
}

#[derive(Debug)]
pub struct Lexer<Chars: Iterator<Item=char>> {
    source: Peekable<Chars>,
}

impl<Chars: Iterator<Item=char>> Lexer<Chars> {
    fn create(chars: Chars) -> Self {
        Self {
            source: chars.peekable(),
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

}

impl<Chars: Iterator<Item=char>> Iterator for Lexer<Chars> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next() {
            Some(c) => {
                match c {
                    '.' => Some(Token::Dot),
                    '|' => Some(Token::Bar),
                    '[' => Some(Token::OpenBracket),
                    ']' => Some(Token::CloseBracket),
                    '"' => {
                        let mut string = String::new();
                        while let Some(c) = self.source.next() {
                            if '"' == c {
                                return Some(Token::Str(string));
                            }
                            string.push(c);
                        }
                        Some(Token::Any(c))
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
                            return Some(Token::Number(word.iter().collect()));
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
                            return Some(Token::Word(word.iter().collect()));
                        }

                        if !c.is_whitespace() {
                            return Some(Token::Any(c));
                        }

                        self.next()
                    }
                }
            }

            None => None
        }
    }
}

impl Lexer<std::vec::IntoIter<char>> {
    pub fn new(expr: &str) -> Self {
        let chr = expr.chars().collect::<Vec<_>>().into_iter();
        Self::create(chr)
    }
}
