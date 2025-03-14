use crate::token::*;
use crate::lexer::*;

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
            Token{token_type: TokenType::NEQ, literal: "!=".into()},
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