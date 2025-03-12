
#[derive(Debug)]
pub enum LexerReadResult {
    Ok,
    EOF,
}

pub struct Lexer {
    pub input: String,
    pub chars: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub character: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input.clone(),
            chars: input.chars().collect(),
            position: 0,
            read_position: 0,
            character: None,
        };
        l.read_char();
        return l;
    }
    pub fn read_char(&mut self) {
        if self.read_position >= self.chars.len() {
            self.character = None;
            return;
        }
        self.character = Some(self.chars[self.read_position]);
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn next_token(&mut self) -> Token {
        match self.character {
            Some(character) => {
                self.read_char();
                match character {
                    '=' => Token{
                        token_type: TokenType::ASSIGN,
                        literal: character.to_string(),
                    },
                    '+' => Token{
                        token_type: TokenType::PLUS,
                        literal: character.to_string(),
                    },
                    '(' => Token{
                        token_type: TokenType::LPAR,
                        literal: character.to_string(),
                    },
                    ')' => Token{
                        token_type: TokenType::RPAR,
                        literal: character.to_string(),
                    },
                    '{' => Token{
                        token_type: TokenType::LBRA,
                        literal: character.to_string(),
                    },
                    '}' => Token{
                        token_type: TokenType::RBRA,
                        literal: character.to_string(),
                    },
                    ',' => Token{
                        token_type: TokenType::COMMA,
                        literal: character.to_string(),
                    },
                    ';' => Token{
                        token_type: TokenType::SEMICOLON,
                        literal: character.to_string(),
                    },
                    _ => Token{
                        token_type: TokenType::ILLEGAL,
                        literal: character.to_string(),
                    }
                }
            }
            None => Token {
                token_type: TokenType::EOF,
                literal: "".into()
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAR,
    RPAR,
    LBRA,
    RBRA,

    // kewords
    FUNCTION,
    LET,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = vec![
            Token{token_type: TokenType::ASSIGN, literal: "=".into()},
            Token{token_type: TokenType::PLUS, literal: "+".into()},
            Token{token_type: TokenType::LPAR, literal: "(".into()},
            Token{token_type: TokenType::RPAR, literal: ")".into()},
            Token{token_type: TokenType::LBRA, literal: "{".into()},
            Token{token_type: TokenType::RBRA, literal: "}".into()},
            Token{token_type: TokenType::COMMA, literal: ",".into()},
            Token{token_type: TokenType::SEMICOLON, literal: ";".into()},
            Token{token_type: TokenType::EOF, literal: "".into()},
        ];

        let mut l = Lexer::new(input.into());
        for test in tests {
            let token = l.next_token();
            assert_eq!(token, test);
        }
    }
}