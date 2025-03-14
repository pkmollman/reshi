
#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    MINUS,
    ASTERISCK,
    SLASH,
    BANG,
    GT,
    LT,
    EQ,
    NOT_EQ,
    COMMA,
    SEMICOLON,
    LPAR,
    RPAR,
    LBRA,
    RBRA,

    // kewords
    FUNCTION,
    LET,
    FALSE,
    TRUE,
    IF,
    ELSE,
    RETURN,
}

pub fn parse_identifier(literal: &String) -> TokenType {
    match literal.as_str() {
        "fn"     => TokenType::FUNCTION,
        "let"    => TokenType::LET,
        "false"  => TokenType::FALSE,
        "true"   => TokenType::TRUE,
        "if"     => TokenType::IF,
        "else"   => TokenType::ELSE,
        "return" => TokenType::RETURN,
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

#[derive(Debug)]
pub enum LexerReadResult {
    Ok,
    EOF,
}

pub struct Lexer {
    pub input: String,
    pub chars: Vec<char>,
    pub position: usize, // points to current char
    pub read_position: usize, // next char
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

    pub fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.chars.len() {
            return None
        } else {
            Some(self.chars[self.read_position])
        }
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
                        '=' => {
                            match self.peek_char() {
                                Some(next_char) => {
                                    if next_char == '=' {
                                        self.read_char();
                                        Token{
                                            token_type: TokenType::EQ,
                                            literal: "==".into(),
                                        }
                                    } else {
                                        Token{
                                            token_type: TokenType::ASSIGN,
                                            literal: character.to_string(),
                                        }
                                    }
                                }
                                None => Token{
                                    token_type: TokenType::ASSIGN,
                                    literal: character.to_string(),
                                }

                            }
                        },
                        '+' => Token{
                            token_type: TokenType::PLUS,
                            literal: character.to_string(),
                        },
                        '-' => Token{
                            token_type: TokenType::MINUS,
                            literal: character.to_string(),
                        },
                        '!' => {
                            match self.peek_char() {
                                Some(next_char) => {
                                    if next_char == '=' {
                                        self.read_char();
                                        Token{
                                            token_type: TokenType::NOT_EQ,
                                            literal: "!=".into(),
                                        }
                                    } else {
                                        Token{
                                            token_type: TokenType::BANG,
                                            literal: character.to_string(),
                                        }
                                    }
                                }
                                None => Token{
                                    token_type: TokenType::BANG,
                                    literal: character.to_string(),
                                }

                            }
                        },
                        '/' => Token{
                            token_type: TokenType::SLASH,
                            literal: character.to_string(),
                        },
                        '*' => Token{
                            token_type: TokenType::ASTERISCK,
                            literal: character.to_string(),
                        },
                        '<' => Token{
                            token_type: TokenType::LT,
                            literal: character.to_string(),
                        },
                        '>' => Token{
                            token_type: TokenType::GT,
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

        println!("token: {}", token.literal);
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

            !-/*5;

            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;

            10 != 9;
            
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

            Token{token_type: TokenType::BANG, literal: "!".into()},
            Token{token_type: TokenType::MINUS, literal: "-".into()},
            Token{token_type: TokenType::SLASH, literal: "/".into()},
            Token{token_type: TokenType::ASTERISCK, literal: "*".into()},
            Token{token_type: TokenType::INT, literal: "5".into()},
            Token{token_type: TokenType::SEMICOLON, literal: ";".into()},

            Token{token_type: TokenType::INT, literal: "5".into()},
            Token{token_type: TokenType::LT, literal: "<".into()},
            Token{token_type: TokenType::INT, literal: "10".into()},
            Token{token_type: TokenType::GT, literal: ">".into()},
            Token{token_type: TokenType::INT, literal: "5".into()},
            Token{token_type: TokenType::SEMICOLON, literal: ";".into()},

            Token{token_type: TokenType::IF, literal: "if".into()},
            Token{token_type: TokenType::LPAR, literal: "(".into()},
            Token{token_type: TokenType::INT, literal: "5".into()},
            Token{token_type: TokenType::LT, literal: "<".into()},
            Token{token_type: TokenType::INT, literal: "10".into()},
            Token{token_type: TokenType::RPAR, literal: ")".into()},
            Token{token_type: TokenType::LBRA, literal: "{".into()},
            Token{token_type: TokenType::RETURN, literal: "return".into()},
            Token{token_type: TokenType::TRUE, literal: "true".into()},
            Token{token_type: TokenType::SEMICOLON, literal: ";".into()},
            Token{token_type: TokenType::RBRA, literal: "}".into()},
            Token{token_type: TokenType::ELSE, literal: "else".into()},
            Token{token_type: TokenType::LBRA, literal: "{".into()},
            Token{token_type: TokenType::RETURN, literal: "return".into()},
            Token{token_type: TokenType::FALSE, literal: "false".into()},
            Token{token_type: TokenType::SEMICOLON, literal: ";".into()},
            Token{token_type: TokenType::RBRA, literal: "}".into()},

            Token{token_type: TokenType::INT, literal: "10".into()},
            Token{token_type: TokenType::EQ, literal: "==".into()},
            Token{token_type: TokenType::INT, literal: "10".into()},
            Token{token_type: TokenType::SEMICOLON, literal: ";".into()},

            Token{token_type: TokenType::INT, literal: "10".into()},
            Token{token_type: TokenType::NOT_EQ, literal: "!=".into()},
            Token{token_type: TokenType::INT, literal: "9".into()},
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