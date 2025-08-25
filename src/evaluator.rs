use std::{cell::RefCell, collections::HashMap};

use crate::parser::{Program, Statement, Statlist};

pub fn apply(program: Program) {
    let env: RefCell<HashMap<String, Box<dyn Fn()>>> = RefCell::new(HashMap::new());
    env.borrow_mut().insert("print".to_string(), Box::new(|| { println!(""); }));
    for funcdef in &program.funcdefs {
        env.borrow_mut().insert(funcdef.name.0.clone(), Box::new( || { eval_statelist(&funcdef.statlist); }));
    }
    env.borrow().get("main").unwrap()();
}

fn eval_statelist(statlist: &Statlist) {
    for statement in &statlist.statements {
        eval_statement(statement);
    }
}

fn eval_statement(statement: &Statement) {
    let Statement::CallFunc(callfunc) = statement;
}