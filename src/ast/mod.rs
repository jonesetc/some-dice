//! Types used to represent the AST of the AnyDice language
//!
//! In general, they are the output of the parser and never handled directly.
//! If creating without  parsing, the functions in [`functional`] offer a simpler interface.

pub mod functional;

/// Operators which have only one operand
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnaryOperator {
    /// Negation: `(-1) = -1`, `(--2) = 2`, `(-0) = 0`
    Negate,
    /// Boolean Not: `(!0) = 1`, `(!1) = 0`, `(!-2) = 0`
    Not,
    /// length of sequence, number of dice, digits in number: `(#{2,4,6}) = 3`, `(#(3d6)) = 3`, `(#123) = 3`
    Length,
}

/// Operators with a left and right operand
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryOperator {
    /// Exponentiation: `(2 ^ 3) = 8`
    Exponent,
    /// Multiplication: `(2 * 3) = 6`
    Multiply,
    /// Integer Division (rounding toward zero): `6 / 3  = 2`, `5 / 3 = 1`, `-5 / 3 = -1`
    Divide,
    /// Addition: `(1 + 2) = 3`
    Add,
    /// Subtraction: `(3 - 2) = 1`
    Subtract,
    /// Boolean And: `(1 & 1) = 1`, `(1 & 0) = 0`, `(3 & -2) = 1`
    And,
    /// Boolean Or: `(0 | 1) = 1`, `(0 | 0) = 0`, `(3 | -2) = 1`
    Or,
    /// Is Equal To: `(2 = 2) = 1`, `(2 = 1) = 0`
    Equal,
    /// Is Not Equal To: `(2 != 1) = 1`, `(2 != 2) = 0`
    NotEqual,
    /// Is Less Than: `(1 < 2) = 1`, `(2 < 2) = 0`, `(3 < 2) = 0`
    Less,
    /// Is Greater Than: `(3 > 2) = 1`, `(2 > 2) = 0`, `(1 > 2) = 0`
    Greater,
    /// Is Less Than Or Equal To: `(1 <= 2) = 1`, `(2 <= 2) = 1`, `(3 <= 2) = 0`
    LessOrEqual,
    /// Is Greater Than Or Equal To: `(3 >= 2) = 1`, `(2 >= 2) = 1`, `(1 >= 2) = 0`
    GreaterOrEqual,
    /// Create a collection of Dice.
    /// The left operand is the number of dice and the right is the description of each die.
    /// We do not distinguish between a single die and a collection of 1.
    Dice,
    /// Get value at index.
    /// The left operand is the index (1-based) and the right is the collection.
    /// Any index out of range returns 0.
    ///
    /// If the index is a sequence, sum results for each value as a separate access: `({1..N}@X) = (1@X) + ... + (N@X)`
    ///
    /// /// <div class="warning">
    ///
    /// Although `left` (index) accepts a generic expression,
    /// it must not evaluate to dice or there will be a runtime error.
    ///
    /// </div>
    ///
    /// If the collection is a sequence, get that index: `(1@{2,4,6}) = 2`, `(3@{2,4,6}) = 6`
    ///
    /// If the collection is dice, order the rolls (descending by default) and get the index: `(1@3d6) = d{1:1, 2:7, 3:19, 4:37, 5:61, 6:91}`, `(3@3d6) = d{1:91, 2:61, 3:37, 4:19, 5:7, 6:1}`
    ///
    /// If the collection is a number, get the digit at that index based on significance (highest by default): `(1@246) = 2`, `31@246) = 6`
    Access,
}

/// An entry in a sequence expression
///
/// <div class="warning">
///
/// Although `repetitions` accepts a generic expression,
/// it must evaluate to a single number or there will be a runtime error.
///
/// </div>
#[derive(Clone, Debug, PartialEq)]
pub enum SequenceEntry {
    /// A single expression to include in a sequence and the number of times to repeat it
    Single {
        value: Expression,
        repetitions: Expression,
    },
    /// A range between (inclusive) expressions to include in a sequence and the number of times to repeat it
    ///
    /// <div class="warning">
    ///
    /// Although `start` and `end` accept a generic expressions,
    /// both must evaluate to a single number or there will be a runtime error.
    ///
    /// </div>
    Range {
        start: Expression,
        end: Expression,
        repetitions: Expression,
    },
}

/// An expression evaluates always to a single value
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    /// An Integer literal
    ///
    /// Capped to i32 as the AnyDice reference server appears to use double-precisions floats.
    /// This means that a loss of precision for integers is possible starting at (positive or negative) `2^53`.
    /// Unfortunately i32 is much below that limit and i64 is much above, but this keeps things simple and will work for almost all common usage.
    Integer { value: i32 },
    /// Name of variable to look up and substiture value of at runtime
    VariableReference { name: String },
    /// An ordered sequence of possible values
    Sequence { entries: Vec<SequenceEntry> },
    /// Operations with only one operand
    UnaryOperation {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    /// Operations with a left and right operand
    BinaryOperation {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// Calls to a user-defined or built-in function
    ///
    /// Name should be normalized by replacing positional arguments with `?`.
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

/// An expected type for a function parameter
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FunctionParameterExpectedType {
    Dice,
    Number,
    Sequence,
}

/// A named parameter for a function definition with an optional expected type
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionParameter {
    name: String,
    expected_type: Option<FunctionParameterExpectedType>,
}

/// A statement alters the state of the interpreter, but does not produce a value
#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    /// Output an expression with an optional name
    Output {
        value: Expression,
        name: Option<String>,
    },
    /// Configure the interpreter behavior with a string value
    ConfigureString { setting: String, value: String },
    /// Configure the interpreter behavior with an expression value
    ConfigureExpression { setting: String, value: Expression },
    /// Define a new or redefine an existing function
    ///
    /// Name should be normalized by replacing positional parameters with `?`.
    FunctionDefinition {
        name: String,
        parameters: Vec<FunctionParameter>,
    },
}

/// The root node of the AST, representing a full program to run
#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    // A list of all statements to be run in order
    pub statements: Vec<Statement>,
}
