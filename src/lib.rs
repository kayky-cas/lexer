#[derive(Debug, PartialEq, Clone, Copy)]
enum BracketState {
    Open,
    Close,
}

enum BracketError {
    UnexpectedClose(char),
    UnexpectedOpen(char),
}

impl Display for BracketError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BracketError::UnexpectedClose(c) => write!(f, "Unexpected close bracket: {}", c),
            BracketError::UnexpectedOpen(c) => write!(f, "Unexpected open bracket: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TokenType {
    Paren(BracketState),
    Curly(BracketState),
    Square(BracketState),
    Let,
    Fn,
    Colon,
    Arrow,
    Assign,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Ident,
    Integer,
    Bigger,
    Smaller,
    Mut,
    Eof,
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

use std::fmt::Display;

use TokenType::*;

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.source.len() {
            if let Some(brace) = self.braces_stack.pop() {
                let brace_error = match brace {
                    Paren(_) => BracketError::UnexpectedOpen('('),
                    Curly(_) => BracketError::UnexpectedOpen('{'),
                    Square(_) => BracketError::UnexpectedOpen('['),
                    _ => unreachable!(),
                };

                panic!("{}", brace_error);
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
                self.braces_stack.push(token.token_type);
                token
            }
            b')' => {
                let token = Token::new(Paren(BracketState::Close), &slice[..1]);
                if let Some(Paren(BracketState::Open)) = self.braces_stack.pop() {
                    token
                } else {
                    panic!("{}", BracketError::UnexpectedClose(')'));
                }
            }
            b'{' => {
                let token = Token::new(Curly(BracketState::Open), &slice[..1]);
                self.braces_stack.push(token.token_type);
                token
            }
            b'}' => {
                let token = Token::new(Curly(BracketState::Close), &slice[..1]);
                if let Some(Curly(BracketState::Open)) = self.braces_stack.pop() {
                    token
                } else {
                    panic!("{}", BracketError::UnexpectedClose('}'));
                }
            }
            b'[' => {
                let token = Token::new(Square(BracketState::Open), &slice[..1]);
                self.braces_stack.push(token.token_type);
                token
            }
            b']' => {
                let token = Token::new(Square(BracketState::Close), &slice[..1]);
                if let Some(Square(BracketState::Open)) = self.braces_stack.pop() {
                    token
                } else {
                    panic!("{}", BracketError::UnexpectedClose(']'));
                }
            }
            b'<' => Token::new(Smaller, &slice[..1]),
            b'>' => Token::new(Bigger, &slice[..1]),
            b',' => Token::new(Comma, &slice[..1]),
            b'.' => Token::new(Dot, &slice[..1]),
            b'-' => {
                let old = self.position;
                self.position += 1;
                if let Some(Token {
                    token_type: Bigger, ..
                }) = self.next()
                {
                    return Some(Token::new(Arrow, &slice[..2]));
                } else {
                    self.position = old;
                    Token::new(Minus, &slice[..1])
                }
            }
            b'+' => Token::new(Plus, &slice[..1]),
            b';' => Token::new(Semicolon, &slice[..1]),
            b'/' => Token::new(Slash, &slice[..1]),
            b'*' => Token::new(Star, &slice[..1]),
            b'=' => Token::new(Assign, &slice[..1]),
            b':' => Token::new(Colon, &slice[..1]),
            b'\0' => Token::new(Eof, &slice[..1]),
            b'a'..=b'z' | b'A'..=b'Z' => {
                let mut end = 1;
                while end < slice.len() && slice[end].is_ascii_alphanumeric() {
                    end += 1;
                }

                let token_type = match &slice[..end] {
                    b"let" => Let,
                    b"mut" => Mut,
                    b"fn" => Fn,
                    _ => Ident,
                };

                Token::new(token_type, &slice[..end])
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

    #[test]
    fn test_a_declare_a_variable() {
        let input = r"let mut five = 5;";

        let expected = vec![
            TokenType::Let,
            TokenType::Mut,
            TokenType::Ident,
            TokenType::Assign,
            TokenType::Integer,
            TokenType::Semicolon,
        ];

        test_lexer(input, expected);
    }

    #[test]
    #[should_panic(expected = "Unexpected close bracket: }")]
    fn test_panic_on_uncorrect_brackets() {
        let input = r"let mut five = 5; }";

        let expected = vec![
            TokenType::Let,
            TokenType::Mut,
            TokenType::Ident,
            TokenType::Assign,
            TokenType::Integer,
            TokenType::Semicolon,
        ];

        test_lexer(input, expected);
    }

    #[test]
    fn test_function() {
        let input = r"fn add(x: int, y: int) -> int {
            x + y
        }";

        let expected = vec![
            TokenType::Fn,
            TokenType::Ident,
            TokenType::Paren(BracketState::Open),
            TokenType::Ident,
            TokenType::Colon,
            TokenType::Ident,
            TokenType::Comma,
            TokenType::Ident,
            TokenType::Colon,
            TokenType::Ident,
            TokenType::Paren(BracketState::Close),
            TokenType::Arrow,
            TokenType::Ident,
            TokenType::Curly(BracketState::Open),
            TokenType::Ident,
            TokenType::Plus,
            TokenType::Ident,
            TokenType::Curly(BracketState::Close),
        ];

        test_lexer(input, expected);
    }

    #[test]
    fn test_arrow() {
        let inputs = vec!["->", "=>", "->>", "->>>", "-->"];
        let expected = vec![
            vec![TokenType::Arrow],
            vec![TokenType::Assign, TokenType::Bigger],
            vec![TokenType::Arrow, TokenType::Bigger],
            vec![TokenType::Arrow, TokenType::Bigger, TokenType::Bigger],
            vec![TokenType::Minus, TokenType::Arrow],
        ];

        for idx in 0..inputs.len() {
            test_lexer(inputs[idx], expected[idx].clone())
        }
    }
}
