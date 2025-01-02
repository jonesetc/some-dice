use std::error;
use std::fmt;

use rand::distributions::WeightedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleError {
    /// The output does not contain any outcomes
    NoOutcomes,
    /// The output contains too many outcomes
    TooManyOutcomes,
    /// The output contains only outcomes with weight of zero
    NoWeight,
    /// The output contains weights that overflow
    TooMuchWeight,
    /// The output contains an outcome with an invalid weight
    /// This should not be possible using public methods
    InvalidWeight,
}

impl error::Error for SampleError {}

impl fmt::Display for SampleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            SampleError::NoOutcomes => "The output does not contain any outcomes",
            SampleError::TooManyOutcomes => "The output contains too many outcomes",
            SampleError::NoWeight => "The output contains only outcomes with weight of zero",
            SampleError::TooMuchWeight => "The output contains weights that overflow",
            SampleError::InvalidWeight => "The output contains an outcome with an invalid weight",
        })
    }
}

impl From<WeightedError> for SampleError {
    fn from(value: WeightedError) -> Self {
        match value {
            WeightedError::NoItem => SampleError::NoOutcomes,
            WeightedError::TooMany => SampleError::TooManyOutcomes,
            WeightedError::AllWeightsZero => SampleError::NoWeight,
            WeightedError::InvalidWeight => SampleError::InvalidWeight,
        }
    }
}
