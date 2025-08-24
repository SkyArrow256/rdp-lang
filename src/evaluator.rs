use std::collections::HashMap;

use crate::parser::{FuncDef, Program, Statlist};

pub struct Evaluator {
    ast: Program,
    env: HashMap<String, Box<dyn Fn(&str)>>,
}

impl Evaluator {
    pub fn new(ast: Program) -> Self {
        Self { ast, env: HashMap::new() }
    }
    pub fn apply(mut self) {
        //print文を定義
        let helloworld = |str: &str| println!("{str}");
        self.env.insert("print".to_string(), Box::new(helloworld));

        eval(&mut self.env, &self.ast);
        //型安全のためJSと同じ書き方ができない！困った(´・ω・`)
        self.env.get("main").unwrap()("");
    }
}

fn eval(env: &mut HashMap<String, Box<dyn Fn(&str)>>, ast: &Program) {
    for funcdef in &ast.funcdefs {
        eval_funcdef(env, funcdef);
    }
}

fn eval_funcdef<'a>(env: &'a mut HashMap<String, Box<dyn Fn(&str)>>, funcdef: &'a FuncDef) {
    env.insert(funcdef.name.0.clone(), Box::new( |_: &str| { eval_statlist(&funcdef.statlist); } ));
}

fn eval_statlist(statlist: &Statlist) {
    for statement in &statlist.statements {

    }
}