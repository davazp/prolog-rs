use crate::terms::{Term, Variable};
use crate::unify::substitute;
use std::collections::HashMap;

use crate::printer;

#[derive(Debug)]
pub struct Env {
    frames: Vec<HashMap<Variable, Term>>,
}

impl Env {
    pub fn new() -> Self {
        Env { frames: Vec::new() }
    }

    pub fn push_frame(&mut self) {
        let frame = HashMap::new();
        self.frames.push(frame)
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }
    pub fn bind(&mut self, variable: Variable, term: Term) {
        let tos = self.frames.last_mut().unwrap();
        tos.insert(variable, term);
    }
    pub fn lookup(&self, variable: &Variable) -> Option<&Term> {
        for frame in self.frames.iter() {
            let binding = frame.get(variable);
            if binding.is_some() {
                return binding;
            }
        }
        None
    }

    pub fn print(&self) {
        for frame in self.frames.iter() {
            for variable in frame.keys() {
                let Variable(name, i) = variable;
                if *i == 0 {
                    let binding = self.lookup(variable).unwrap();
                    let value = substitute(&self, binding);
                    println!("{} = {}", name, printer::print(&value));
                }
            }
        }
    }
}
