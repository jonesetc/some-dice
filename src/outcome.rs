//! The result of [interpretation][0], used for [sampling][1]
//!
//! [0]: super::interpret
//! [1]: super::sample

/// A value with a weight
#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
    pub value: i32,
    pub weight: usize,
}
