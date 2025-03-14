use crate::ast::*;


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
    NEQ,
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

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: dyn Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

type Identifier = Token;

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.literal.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}