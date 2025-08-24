use crate::scanner::{Scanner, Token};

pub struct Parser<'a> {
	scanner: Scanner<'a>
}

impl<'a> Parser<'a> {
	pub fn with_scanner(scanner: Scanner<'a>) -> Self {
		Self { scanner }
	}
	pub fn parse(self) -> Program {
		let ast = self.program();
		ast
	}
	fn program(&mut self) -> Program {
		let program = Program::FuncDef;
		while self.scanner.is_match(Token::FuncDef) {
			
		}
		program
	}
	fn funcdef(&mut self) -> Program {

	}
	fn funcargs(&mut self) -> Program {

	}
	fn statlist(&mut self) -> Program {

	}
	fn statement(&mut self) -> Program {
		if let Some(Token::Ident(name)) = self.scanner.take() {
			//セミコロンを取る
			self.scanner.take();
		} else {
			panic!();
		}
	}
	fn call_func(&mut self) -> Program {
		
	}
}

struct Program{
	funcdefs: Vec<FuncDef>,
}
struct FuncDef {
	name: Ident,
	args: Funcargs,
	statlist: Statlist,
}

struct Funcargs {
	idents: Vec<Ident>,
}

struct Statlist {
	statements: Vec<Statement>,
}

enum Statement {
	CallFunc(CallFunc),
}

struct CallFunc {
	name: Ident,
	arg: Str,
}

struct Str(String);

struct Ident(String);