#[derive(Debug, PartialEq, Clone, Copy)]
enum BracketState {
    Open,
    Close,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TokenType {
    Paren(BracketState),
    Curly(BracketState),
    Square(BracketState),
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    EOF,
    Word,
    Integer,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token<'a> {
    token_type: TokenType,
    literal: &'a [u8],
}

impl Token<'_> {
    fn new(token_type: TokenType, literal: &[u8]) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}

pub struct Lexer<'a> {
    source: &'a [u8],
    position: usize,
    braces_stack: Vec<TokenType>,
}

impl Lexer<'_> {
    pub fn new(source: &[u8]) -> Lexer {
        Lexer {
            source,
            position: 0,
            braces_stack: Vec::new(),
        }
    }
}

use TokenType::*;

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.source.len() {
            if !self.braces_stack.is_empty() {
                panic!("Unexpected EOF");
            }

            return None;
        }

        let slice = &self.source[self.position..];

        let token = match slice[0] {
            b' ' | b'\n' | b'\t' => {
                self.position += 1;
                return self.next();
            }
            b'(' => {
                let token = Token::new(Paren(BracketState::Open), &slice[..1]);
                self.braces_stack.push(token.token_type.clone());
                token
            }
            b')' => {
                let token = Token::new(Paren(BracketState::Close), &slice[..1]);
                if let Some(Paren(BracketState::Open)) = self.braces_stack.pop() {
                    token
                } else {
                    panic!("Unexpected ')'");
                }
            }
            b'{' => {
                let token = Token::new(Curly(BracketState::Open), &slice[..1]);
                self.braces_stack.push(token.token_type.clone());
                token
            }
            b'}' => {
                let token = Token::new(Curly(BracketState::Close), &slice[..1]);
                if let Some(Curly(BracketState::Open)) = self.braces_stack.pop() {
                    token
                } else {
                    panic!("Unexpected '}}'");
                }
            }
            b'[' => {
                let token = Token::new(Square(BracketState::Open), &slice[..1]);
                self.braces_stack.push(token.token_type.clone());
                token
            }
            b']' => {
                let token = Token::new(Square(BracketState::Close), &slice[..1]);
                if let Some(Square(BracketState::Open)) = self.braces_stack.pop() {
                    token
                } else {
                    panic!("Unexpected ']'");
                }
            }
            b',' => Token::new(Comma, &slice[..1]),
            b'.' => Token::new(Dot, &slice[..1]),
            b'-' => Token::new(Minus, &slice[..1]),
            b'+' => Token::new(Plus, &slice[..1]),
            b';' => Token::new(Semicolon, &slice[..1]),
            b'/' => Token::new(Slash, &slice[..1]),
            b'*' => Token::new(Star, &slice[..1]),
            b'\0' => Token::new(EOF, &slice[..1]),
            b'a'..=b'z' | b'A'..=b'Z' => {
                let mut end = 1;
                while end < slice.len() && slice[end].is_ascii_alphanumeric() {
                    end += 1;
                }
                Token::new(Word, &slice[..end])
            }
            b'0'..=b'9' => {
                let mut end = 1;
                while end < slice.len() && slice[end].is_ascii_digit() {
                    end += 1;
                }
                Token::new(Integer, &slice[..end])
            }
            _ => panic!("Unknown token"),
        };

        self.position += token.literal.len();

        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lexer(input: &str, expected: Vec<TokenType>) {
        let lexer = super::Lexer::new(input.as_bytes());

        let tokens: Vec<_> = lexer.map(|token| token.token_type).collect();

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_lexer_paren() {
        test_lexer(
            "()",
            vec![
                TokenType::Paren(BracketState::Open),
                TokenType::Paren(BracketState::Close),
            ],
        );
    }

    #[test]
    fn test_lexer_brace() {
        test_lexer(
            "{}",
            vec![
                TokenType::Curly(BracketState::Open),
                TokenType::Curly(BracketState::Close),
            ],
        );
    }
}
