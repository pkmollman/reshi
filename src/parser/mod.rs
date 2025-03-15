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