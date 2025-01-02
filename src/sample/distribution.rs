use std::array;
use std::iter;

use rand::{distributions::WeightedIndex, prelude::Distribution};

use super::error::SampleError;
use crate::outcome::Outcome;

#[derive(Debug, Clone, PartialEq)]
pub struct OutputDistribution {
    values: Vec<i32>,
    index: WeightedIndex<usize>,
}

impl OutputDistribution {
    pub fn new(outcomes: Vec<Outcome>) -> Result<Self, SampleError> {
        let (values, weights) =
            outcomes
                .iter()
                .fold((vec![], vec![]), |(mut values, mut weights), outcome| {
                    values.push(outcome.value);
                    weights.push(outcome.weight);

                    (values, weights)
                });

        // Check for overflows ahead of time as rand will just let this panic
        if weights
            .iter()
            .try_fold(0usize, |acc, &curr| acc.checked_add(curr))
            .is_none()
        {
            return Err(SampleError::TooMuchWeight);
        };

        // All other common errors are caught by rand and transformed
        let index = WeightedIndex::new(&weights)?;

        Ok(Self { values, index })
    }

    pub fn sample<R: rand::Rng>(&self, rng: &mut R) -> i32 {
        self.values[self.index.sample(rng)]
    }

    pub fn sample_many<R: rand::Rng>(&self, n: usize, rng: &mut R) -> Vec<i32> {
        iter::repeat_with(|| self.sample(rng)).take(n).collect()
    }

    pub fn sample_n<const N: usize, R: rand::Rng>(&self, rng: &mut R) -> [i32; N] {
        array::from_fn(|_| self.sample(rng))
    }

    pub fn sample_iter<'a, R: rand::Rng>(
        &'a self,
        rng: &'a mut R,
    ) -> impl IntoIterator<Item = i32> + use<'a, R> {
        (&self.index).sample_iter(rng).map(|i| self.values[i])
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_new_errors() {
        assert_eq!(
            OutputDistribution::new(vec![]),
            Err(SampleError::NoOutcomes),
        );

        // SampleError::TooManyOutcomes is impractical to test here

        assert_eq!(
            OutputDistribution::new(vec![Outcome {
                value: 1,
                weight: 0,
            }]),
            Err(SampleError::NoWeight),
        );

        assert_eq!(
            OutputDistribution::new(vec![
                Outcome {
                    value: 1,
                    weight: usize::MAX,
                },
                Outcome {
                    value: 2,
                    weight: 1,
                }
            ]),
            Err(SampleError::TooMuchWeight),
        );

        // SampleError::InvalidWeight can't be created using this module
    }

    #[test]
    fn test_sample() {
        let mut rng = rand::thread_rng();
        let value = rng.gen();
        let dist = OutputDistribution::new(vec![Outcome { value, weight: 1 }]).unwrap();

        assert_eq!(dist.sample(&mut rng), value);
    }

    #[test]
    fn test_sample_many() {
        let mut rng = rand::thread_rng();
        let value = rng.gen();
        let dist = OutputDistribution::new(vec![Outcome { value, weight: 1 }]).unwrap();

        assert_eq!(dist.sample_many(5, &mut rng,), vec![value; 5],);
    }

    #[test]
    fn test_sample_n() {
        let mut rng = rand::thread_rng();
        let value = rng.gen();
        let dist = OutputDistribution::new(vec![Outcome { value, weight: 1 }]).unwrap();

        assert_eq!(dist.sample_n(&mut rng), [value; 5],);
    }

    #[test]
    fn test_sample_iter() {
        let mut rng = rand::thread_rng();
        let value = rng.gen();
        let dist = OutputDistribution::new(vec![Outcome { value, weight: 1 }]).unwrap();

        assert_eq!(
            dist.sample_iter(&mut rng)
                .into_iter()
                .take(5)
                .collect::<Vec<_>>(),
            vec![value; 5],
        );
    }
}
