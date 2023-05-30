#[derive(Debug, PartialEq)]
enum TokenType {
    Paren,
    Brace,
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
}

impl Lexer<'_> {
    fn new(source: &[u8]) -> Lexer {
        Lexer {
            source,
            position: 0,
        }
    }
}

use TokenType::*;

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.source.len() {
            return None;
        }

        let slice = &self.source[self.position..];

        let token = match slice[0] {
            b'(' => Token::new(Paren, &slice[..1]),
            b')' => Token::new(Paren, &slice[..1]),
            b'{' => Token::new(Brace, &slice[..1]),
            b'}' => Token::new(Brace, &slice[..1]),
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
    use super::TokenType;

    fn test_lexer(input: &str, expected: Vec<TokenType>) {
        let lexer = super::Lexer::new(input.as_bytes());

        let tokens: Vec<_> = lexer.map(|token| token.token_type).collect();

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_lexer_paren() {
        test_lexer("()", vec![TokenType::Paren, TokenType::Paren]);
    }
}
