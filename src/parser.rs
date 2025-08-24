use crate::scanner::{Scanner, Token};

// トークンをtakeするときに型の確認とかできるようにした方が絶対いいと思った。
// 実際に値でない部分（かっことかセミコロンとか）のトークンが間違ってても気づかないし...

pub struct Parser {
    scanner: Scanner,
}

impl Parser {
    pub fn with_scanner(scanner: Scanner) -> Self {
        Self { scanner }
    }
    pub fn parse(mut self) -> Program {
        let ast = self.program();
        ast
    }
    fn program(&mut self) -> Program {
        let mut funcdefs = Vec::new();
        while self.scanner.is_match(Token::FuncDef) {
            funcdefs.push(self.funcdef());
        }
        Program { funcdefs }
    }
    fn funcdef(&mut self) -> FuncDef {
        self.scanner.take().unwrap(); //FuncDefを取る
        if let Some(Token::Ident(name)) = self.scanner.take() {
            self.scanner.take().unwrap(); //(を取る
            let args = self.funcargs();
            self.scanner.take().unwrap(); //)を取る
            self.scanner.take().unwrap(); //{を取る
            let statlist = self.statlist();
            //self.scanner.take().unwrap(); //}を取る
            FuncDef {
                name: Ident(name),
                args,
                statlist,
            }
        } else {
            panic!("予期されていないトークンです");
        }
    }
    fn funcargs(&mut self) -> Funcargs {
        Funcargs {}
    }
    fn statlist(&mut self) -> Statlist {
        let mut statlist = Statlist {
            statements: Vec::new(),
        };
        while self.scanner.is_match(Token::Ident("".to_string())) {
            println!("命令が見つかりました");
            statlist.statements.push(self.statement());
        }
        statlist
    }
    fn statement(&mut self) -> Statement {
        if let Some(Token::Ident(name)) = self.scanner.take() {
            let statement = Statement::CallFunc(self.call_func(Ident(name)));
            self.scanner.take(); //セミコロンを取る
            statement
        } else {
            panic!("予期されていないトークンです");
        }
    }
    fn call_func(&mut self, name: Ident) -> CallFunc {
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

#[derive(Debug)]
pub struct Program {
    pub funcdefs: Vec<FuncDef>,
}

#[derive(Debug)]
pub struct FuncDef {
    pub name: Ident,
    pub args: Funcargs,
    pub statlist: Statlist,
}

#[derive(Debug)]
pub struct Funcargs {}

#[derive(Debug)]
pub struct Statlist {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    CallFunc(CallFunc),
}

#[derive(Debug)]
pub struct CallFunc {
    pub name: Ident,
    pub arg: Str,
}

#[derive(Debug)]
pub struct Str(String);

#[derive(Debug)]
pub struct Ident(pub String);
