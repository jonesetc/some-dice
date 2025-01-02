use super::{config, env, output};
use crate::ast;

/// An interpreter for AnyDice with state for configuration, variable, functions, and outputs
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Interpreter {
    environment: env::Environment,
    configuration: config::Configuration,
}

impl Interpreter {
    pub fn new() -> Self {
        Default::default()
    }

    /// Evaluate many statements
    pub fn run_program(&mut self, program: ast::Program) {
        todo!("Run a full program")
    }

    /// Evaluate a single statement
    pub fn run_statement(&mut self, statement: ast::Statement) {
        todo!("Run a single statement")
    }

    /// Evaluate a single expression
    pub fn run_expression(&self, expression: ast::Expression) -> output::Output {
        todo!("Run a single Expression")
    }
}
