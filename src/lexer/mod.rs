
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
    pub fn eat_whitespace(&mut self) {
        while let Some(character) = self.character {
            if character.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
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
        self.eat_whitespace();
        let token = match self.character {
            Some(character) => {
                if character.is_digit(10) {
                    self.read_number()
                } else if character.is_alphabetic() {
                    self.read_identifier()
                } else {
                    let token = match character {
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
                    };
                    self.read_char();
                    token
                }
            }
            None => Token {
                token_type: TokenType::EOF,
                literal: "".into()
            }
        };
        token
    }
    pub fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while let Some(character) = self.character {
            if !character.is_alphabetic() {
                break;
            }
            self.read_char();
        }
        let literal = self.input[position..self.position].to_string();
        return Token {
            token_type: parse_identifier(&literal),
            literal: literal,
        }
    }
    pub fn read_number(&mut self) -> Token {
        let position = self.position;
        while let Some(character) = self.character {
            println!("{}", character);
            if !character.is_digit(10) {
                break;
            }
            self.read_char();
        }
        let literal = self.input[position..self.position].to_string();
        return Token {
            token_type: TokenType::INT,
            literal: literal,
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

pub fn parse_identifier(literal: &String) -> TokenType {
    match literal.as_str() {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        _ => TokenType::IDENT,
    }
}

impl TokenType {
    pub fn is_keyword(&self) -> bool {
        match self {
            TokenType::FUNCTION => true,
            TokenType::LET => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"

            let five = 5;
            
            let ten = 10;
            
            let add = fn(x, y) {
                x + y;
            };
            
            let result = add(five, ten);
            
        "#;

        let tests = vec![
            Token{token_type: TokenType::LET, literal: "let".into()},
            Token{token_type: TokenType::IDENT, literal: "five".into()},
            Token{token_type: TokenType::ASSIGN, literal: "=".into()},
            Token{token_type: TokenType::INT, literal: "5".into()},
            Token{token_type: TokenType::SEMICOLON, literal: ";".into()},
            Token{token_type: TokenType::LET, literal: "let".into()},
            Token{token_type: TokenType::IDENT, literal: "ten".into()},
            Token{token_type: TokenType::ASSIGN, literal: "=".into()},
            Token{token_type: TokenType::INT, literal: "10".into()},
            Token{token_type: TokenType::SEMICOLON, literal: ";".into()},
            Token{token_type: TokenType::LET, literal: "let".into()},
            Token{token_type: TokenType::IDENT, literal: "add".into()},
            Token{token_type: TokenType::ASSIGN, literal: "=".into()},
            Token{token_type: TokenType::FUNCTION, literal: "fn".into()},
            Token{token_type: TokenType::LPAR, literal: "(".into()},
            Token{token_type: TokenType::IDENT, literal: "x".into()},
            Token{token_type: TokenType::COMMA, literal: ",".into()},
            Token{token_type: TokenType::IDENT, literal: "y".into()},
            Token{token_type: TokenType::RPAR, literal: ")".into()},
            Token{token_type: TokenType::LBRA, literal: "{".into()},
            Token{token_type: TokenType::IDENT, literal: "x".into()},
            Token{token_type: TokenType::PLUS, literal: "+".into()},
            Token{token_type: TokenType::IDENT, literal: "y".into()},
            Token{token_type: TokenType::SEMICOLON, literal: ";".into()},
            Token{token_type: TokenType::RBRA, literal: "}".into()},
            Token{token_type: TokenType::SEMICOLON, literal: ";".into()},
            Token{token_type: TokenType::LET, literal: "let".into()},
            Token{token_type: TokenType::IDENT, literal: "result".into()},
            Token{token_type: TokenType::ASSIGN, literal: "=".into()},
            Token{token_type: TokenType::IDENT, literal: "add".into()},
            Token{token_type: TokenType::LPAR, literal: "(".into()},
            Token{token_type: TokenType::IDENT, literal: "five".into()},
            Token{token_type: TokenType::COMMA, literal: ",".into()},
            Token{token_type: TokenType::IDENT, literal: "ten".into()},
            Token{token_type: TokenType::RPAR, literal: ")".into()},
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