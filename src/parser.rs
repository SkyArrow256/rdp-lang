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
	fn funcdef(&mut self) -> FuncDef {

	}
	fn funcargs(&mut self) -> Funcargs {

	}
	fn statlist(&mut self) -> Statlist {
		let mut statlist = Statlist { statements: Vec::new() };
		while self.scanner.is_match(Token::Ident("".to_string())) {
			statlist.statements.push(self.statement());
		}
		statlist
	}
	fn statement(&mut self) -> Statement {
		if let Some(Token::Ident(name)) = self.scanner.take() {
			//セミコロンを取る
			self.scanner.take();
			Statement::CallFunc(self.call_func(Ident(name)))
		} else {
			panic!("予期されていないトークンです")
		}
	}
	fn call_func(&'a mut self, name: Ident) -> CallFunc {
		//かっこを取る
		self.scanner.take().unwrap();
		let arg = self.scanner.take();
		self.scanner.take().unwrap();
		if let Some(Token::String(arg)) = arg {
			let arg = Str(arg);
			CallFunc { name, arg }
		} else {
			panic!("予期されていないトークンです");
		}
	}
}

pub struct Program{
	pub funcdefs: Vec<FuncDef>,
}
pub struct FuncDef {
	pub name: Ident,
	pub args: Funcargs,
	pub statlist: Statlist,
}

pub struct Funcargs {
	pub idents: Vec<Ident>,
}

pub struct Statlist {
	pub statements: Vec<Statement>,
}

pub enum Statement {
	CallFunc(CallFunc),
}

pub struct CallFunc {
	pub name: Ident,
	pub arg: Str,
}

pub struct Str(String);

pub struct Ident(String);