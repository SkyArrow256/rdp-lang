mod scanner;
mod parser;
mod evaluator;

use scanner::Scanner;
use parser::Parser;
use evaluator::Evaluator;

pub fn run(text: &str) {
	let sc = Scanner::from(text);
	println!("{sc:#?}");
	let ast = Parser::with_scanner(sc).parse();
	Evaluator::run(ast);
}