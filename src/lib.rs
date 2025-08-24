mod evaluator;
mod parser;
mod scanner;

use evaluator::Evaluator;
use parser::Parser;
use scanner::Scanner;

pub fn run(text: &str) {
    let sc = Scanner::from(text);
    println!("{sc:#?}");
    let ast = Parser::with_scanner(sc).parse();
    println!("{ast:#?}");
    Evaluator::new(ast).apply();
}
