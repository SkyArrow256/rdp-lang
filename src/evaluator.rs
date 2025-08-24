use std::collections::HashMap;

use crate::parser::Program;

pub struct Evaluator {
    ast: Program,
    env: HashMap<>
}

impl Evaluator {
    pub fn new(ast: Program) -> Self {
        Self { ast }
    }
    pub fn apply(&mut self) {
        println!("Running...");
        let helloworld = |str: &str| println!("{str}");
    }
}
