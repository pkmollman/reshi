use crate::lexer::*;
use crate::token::*;
use crate::ast::*;
use std::{cell::RefCell, rc::Rc};


pub struct Parser {
    lexer: Rc<RefCell<Lexer>>,
    current_token: Option<Token>,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Rc<RefCell<Lexer>>) -> Parser {
        let token1 = lexer.borrow_mut().next_token();
        return Parser{
            lexer,
            current_token: None,
            peek_token: token1,
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = Some(self.peek_token.clone());
        self.peek_token = self.lexer.borrow_mut().next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        return Program{
            statements: Vec::new(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"
        
        let x = 5;

        let y = 10;

        let foobar = 838383;
        
        "#;


        let mut lexer = Lexer::new(input.into());


        let mut parser = Parser::new(Rc::new(RefCell::new(lexer)));

        let mut program = parser.parse_program();

        if program.statements.len() != 3 {
            panic!("program has {} statements, instead of 3", program.statements.len())
        }

    }
}