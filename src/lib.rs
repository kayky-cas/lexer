#[derive(Debug, PartialEq, Clone, Copy)]
enum BracketState {
    Open,
    Close,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TokenType {
    Paren(BracketState),
    Brace(BracketState),
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
}

struct Token<'a> {
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

struct Lexer<'a> {
    source: &'a [u8],
    position: usize,
    braces_stack: Vec<TokenType>,
}

impl Lexer<'_> {
    fn new(source: &[u8]) -> Lexer {
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
                panic!("Mismatched brackets");
            }
            return None;
        }

        let slice = &self.source[self.position..];

        let token = match slice[0] {
            b'(' => {
                let token = Token::new(Paren(BracketState::Open), &slice[..1]);
                self.braces_stack.push(token.token_type.clone());
                token
            }
            b')' => {
                let token = Token::new(Paren(BracketState::Close), &slice[..1]);
                if let Some(bracket) = self.braces_stack.pop() {
                    if bracket != token.token_type {
                        panic!("Mismatched brackets");
                    }
                }
                token
            }
            b'{' => {
                let token = Token::new(Brace(BracketState::Open), &slice[..1]);
                self.braces_stack.push(token.token_type.clone());
                token
            }
            b'}' => {
                let token = Token::new(Brace(BracketState::Close), &slice[..1]);
                if let Some(bracket) = self.braces_stack.pop() {
                    if bracket != token.token_type {
                        panic!("Mismatched brackets");
                    }
                }
                token
            }
            b',' => Token::new(Comma, &slice[..1]),
            b'.' => Token::new(Dot, &slice[..1]),
            b'-' => Token::new(Minus, &slice[..1]),
            b'+' => Token::new(Plus, &slice[..1]),
            b';' => Token::new(Semicolon, &slice[..1]),
            b'/' => Token::new(Slash, &slice[..1]),
            b'*' => Token::new(Star, &slice[..1]),
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
                TokenType::Paren(BracketState::Open),
            ],
        );
    }
}
