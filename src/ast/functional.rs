//! Helper functions to create AST nodes.
//!
//! Each function creates a single AST node.
//!
//! Due to the recursive nature of expressions,
//! the output of expression functions must be boxed before being passed to another expression function.
//!
//! ```
//! // output 1d3 - 2 named "fudge die"
//! // A program with one output: a die with options -1, 0, and 1 named fudge die
//! program(vec![output(
//!     subtract(
//!         Box::new(dice(Box::new(integer(1)), Box::new(integer(3)))),
//!         Box::new(integer(2)),
//!     ),
//!     None,
//!     )])
//! ```

use crate::ast;

macro_rules! unary_operation {
    ($name: ident, $op:expr) => {
        #[doc=concat!("Create [unary operation expression] expression with operator [", stringify!($name), "]")]
        #[doc="\n"]
        #[doc="[unary operation expression]: ast::Expression::UnaryOperation"]
        #[doc=concat!("[", stringify!($name), "]: ", stringify!($op))]
        pub fn $name(operand: Box<ast::Expression>) -> ast::Expression {
            ast::Expression::UnaryOperation {
                operator: $op,
                operand,
            }
        }
    };
}

unary_operation!(negate, ast::UnaryOperator::Negate);
unary_operation!(not, ast::UnaryOperator::Not);
unary_operation!(length, ast::UnaryOperator::Length);

macro_rules! binary_operation {
    ($name: ident, $op:expr) => {
        #[doc=concat!("Create [binary operation expression] expression with operator [", stringify!($name), "]")]
        #[doc="\n"]
        #[doc="[binary operation expression]: ast::Expression::UnaryOperation"]
        #[doc=concat!("[", stringify!($name), "]: ", stringify!($op))]
        pub fn $name(
            left: Box<ast::Expression>,
            right: Box<ast::Expression>,
        ) -> ast::Expression {
            ast::Expression::BinaryOperation {
                operator: $op,
                left,
                right,
            }
        }
    };
}

binary_operation!(exponent, ast::BinaryOperator::Exponent);
binary_operation!(multiply, ast::BinaryOperator::Multiply);
binary_operation!(divide, ast::BinaryOperator::Divide);
binary_operation!(add, ast::BinaryOperator::Add);
binary_operation!(subtract, ast::BinaryOperator::Subtract);
binary_operation!(and, ast::BinaryOperator::And);
binary_operation!(or, ast::BinaryOperator::Or);
binary_operation!(equal, ast::BinaryOperator::Equal);
binary_operation!(not_equal, ast::BinaryOperator::NotEqual);
binary_operation!(less, ast::BinaryOperator::Less);
binary_operation!(less_or_equal, ast::BinaryOperator::LessOrEqual);
binary_operation!(greater, ast::BinaryOperator::Greater);
binary_operation!(greater_or_equal, ast::BinaryOperator::GreaterOrEqual);
binary_operation!(dice, ast::BinaryOperator::Dice);
binary_operation!(access, ast::BinaryOperator::Access);

/// Create [integer expression]
///
/// [integer expression]: ast::Expression::Integer
pub fn integer(value: i32) -> ast::Expression {
    ast::Expression::Integer { value }
}

/// Create [variable reference]
///
/// [variable reference]: ast::Expression::VariableReference
pub fn variable_reference(name: String) -> ast::Expression {
    ast::Expression::VariableReference { name }
}

/// Create [single sequence entry]
///
/// [single sequence entry]: ast::SequenceEntry::Single
pub fn single_entry(value: ast::Expression, repetitions: ast::Expression) -> ast::SequenceEntry {
    ast::SequenceEntry::Single { value, repetitions }
}

/// Create [range sequence entry]
///
/// [range sequence entry]: ast::SequenceEntry::Range
pub fn range_entry(
    start: ast::Expression,
    end: ast::Expression,
    repetitions: ast::Expression,
) -> ast::SequenceEntry {
    ast::SequenceEntry::Range {
        start,
        end,
        repetitions,
    }
}

/// Create [sequence expression]
///
/// [sequence expression]: ast::Expression::Sequence
pub fn sequence(entries: Vec<ast::SequenceEntry>) -> ast::Expression {
    ast::Expression::Sequence { entries }
}

/// Create [output statement]
///
/// [output statement]: ast::Statement::Output
pub fn output(value: ast::Expression, name: Option<String>) -> ast::Statement {
    ast::Statement::Output { value, name }
}

/// Create [configure statement] with string value
///
/// [configure statement]: ast::Statement::ConfigureString
pub fn configure_string(setting: String, value: String) -> ast::Statement {
    ast::Statement::ConfigureString { setting, value }
}

/// Create [configure statement] with expression value
///
/// [configure statement]: ast::Statement::ConfigureExpression
pub fn configure_expression(setting: String, value: ast::Expression) -> ast::Statement {
    ast::Statement::ConfigureExpression { setting, value }
}

/// Create [program]
///
/// [program]: ast::Program
pub fn program(statements: Vec<ast::Statement>) -> ast::Program {
    ast::Program { statements }
}
