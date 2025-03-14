mod lexer;
use crate::lexer::*;

fn main() {
    let stdin = std::io::stdin();
    while true {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let mut l = Lexer::new(buffer);

        let mut token = l.next_token();
        while token.token_type != TokenType::EOF {
            println!("{:?}", token);
            token = l.next_token();
        }
    }
}
