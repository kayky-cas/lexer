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
    EOF,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
    pub fn new(source: &[u8]) -> Lexer {
        Lexer {
            source,
            position: 0,
            braces_stack: Vec::new(),
        }
    }

    pub fn validate(&self) -> bool {
        true
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
                    return None;
                }
            }
            b'{' => {
                let token = Token::new(Brace(BracketState::Open), &slice[..1]);
                self.braces_stack.push(token.token_type.clone());
                token
            }
            b'}' => {
                let token = Token::new(Brace(BracketState::Close), &slice[..1]);
                if let Some(Brace(BracketState::Open)) = self.braces_stack.pop() {
                    token
                } else {
                    return None;
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

        assert_eq!(tokens.last(), Some(&TokenType::EOF));
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
                TokenType::Brace(BracketState::Open),
                TokenType::Brace(BracketState::Close),
            ],
        );
    }

    #[test]
    fn test_faild_bracket() {
        let lexer = super::Lexer::new("(".as_bytes());

        let tokens: Vec<_> = lexer.map(|token| token.token_type).collect();

        assert_eq!(tokens.last(), Some(&TokenType::EOF));
        assert_eq!(tokens, vec![TokenType::Paren(BracketState::Open)]);
    }
}
