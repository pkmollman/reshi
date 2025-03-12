mod lexer;
use crate::lexer::*;

const reshi_script: &str = r#"

let five = 5;

let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);

"#;

fn main() {
    let mut l = Lexer::new(reshi_script.into());
    loop {
        let token = l.next_token();
        match token.token_type {
            TokenType::EOF => {
                break;
            }
            _ => {
                println!("found: {:?}", token)
            }
        }
    }
}
