mod dice;
mod number;
mod sequence;

use super::factor;
use crate::outcome;

#[derive(Clone, Debug, PartialEq)]
pub(super) enum Value {
    Number(number::Number),
    Dice(dice::Dice),
    Sequence(sequence::Sequence),
}

impl From<Value> for Vec<outcome::Outcome> {
    fn from(val: Value) -> Self {
        match val {
            Value::Number(number::Number { value }) => vec![outcome::Outcome {
                value: value,
                weight: 1,
            }],
            Value::Dice(dice) => {
                let die: dice::Die = dice.into();

                if die.chances.is_empty() {
                    vec![outcome::Outcome {
                        value: 0,
                        weight: 1,
                    }]
                } else {
                    // Determing LCM of denominators to scale up to whole weights
                    let lcm = die
                        .chances
                        .iter()
                        .fold(1, |acc, (_, chance)| factor::lcm(acc, chance.denominator));

                    let outcomes = die
                        .chances
                        .iter()
                        .map(|(&side, chance)| outcome::Outcome {
                            value: side,
                            weight: chance.numerator * lcm / chance.denominator,
                        })
                        .collect::<Vec<_>>();

                    // Determine GCD of weights to scale down to smallest whole weights
                    let gcd = outcomes.iter().fold(outcomes[0].weight, |acc, outcome| {
                        factor::gcd(acc, outcome.weight)
                    });

                    outcomes
                        .into_iter()
                        .map(|outcome| outcome::Outcome {
                            value: outcome.value,
                            weight: outcome.weight / gcd,
                        })
                        .collect()
                }
            }
            Value::Sequence(sequence) => {
                if sequence.values.is_empty() {
                    vec![outcome::Outcome {
                        value: 0,
                        weight: 1,
                    }]
                } else {
                    sequence
                        .values
                        .iter()
                        .map(|&value| outcome::Outcome {
                            value: value,
                            weight: 1,
                        })
                        .collect()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpret::chance;

    #[test]
    fn test_number_into_outcomes() {
        assert_eq!(
            Into::<Vec<outcome::Outcome>>::into(Value::Number(number::Number { value: 1 })),
            vec![outcome::Outcome {
                value: 1,
                weight: 1,
            }]
        );
    }

    #[test]
    fn test_dice_into_outcomes() {
        assert_eq!(
            Into::<Vec<outcome::Outcome>>::into(Value::Dice(dice::Dice {
                count: 1,
                die: dice::Die::new(vec![
                    (1, chance::Chance::new(1, 4)),
                    (2, chance::Chance::new(1, 4)),
                    (3, chance::Chance::new(1, 4)),
                    (4, chance::Chance::new(1, 4)),
                ])
            })),
            vec![
                outcome::Outcome {
                    value: 1,
                    weight: 1,
                },
                outcome::Outcome {
                    value: 2,
                    weight: 1,
                },
                outcome::Outcome {
                    value: 3,
                    weight: 1,
                },
                outcome::Outcome {
                    value: 4,
                    weight: 1,
                }
            ]
        );

        assert_eq!(
            Into::<Vec<outcome::Outcome>>::into(Value::Dice(dice::Dice {
                count: 1,
                die: dice::Die::new(vec![
                    (1, chance::Chance::new(1, 2)),
                    (2, chance::Chance::new(1, 3)),
                    (3, chance::Chance::new(1, 6)),
                ])
            })),
            vec![
                outcome::Outcome {
                    value: 1,
                    weight: 3,
                },
                outcome::Outcome {
                    value: 2,
                    weight: 2,
                },
                outcome::Outcome {
                    value: 3,
                    weight: 1,
                },
            ]
        );
    }

    #[test]
    fn test_empty_dice_into_outcomes() {
        assert_eq!(
            Into::<Vec<outcome::Outcome>>::into(Value::Dice(dice::Dice {
                count: 1,
                die: dice::Die::new(vec![])
            })),
            vec![outcome::Outcome {
                value: 0,
                weight: 1,
            }]
        );

        assert_eq!(
            Into::<Vec<outcome::Outcome>>::into(Value::Dice(dice::Dice {
                count: 0,
                die: dice::Die::new(vec![
                    (1, chance::Chance::new(1, 4)),
                    (2, chance::Chance::new(1, 4)),
                    (3, chance::Chance::new(1, 4)),
                    (4, chance::Chance::new(1, 4)),
                ])
            })),
            vec![outcome::Outcome {
                value: 0,
                weight: 1,
            }]
        )
    }

    #[test]
    fn test_sequence_into_outcomes() {
        assert_eq!(
            Into::<Vec<outcome::Outcome>>::into(Value::Sequence(sequence::Sequence {
                values: vec![1, 2, 3, 4]
            })),
            vec![
                outcome::Outcome {
                    value: 1,
                    weight: 1,
                },
                outcome::Outcome {
                    value: 2,
                    weight: 1,
                },
                outcome::Outcome {
                    value: 3,
                    weight: 1,
                },
                outcome::Outcome {
                    value: 4,
                    weight: 1,
                }
            ]
        );
    }

    #[test]
    fn test_empty_sequence_into_outcomes() {
        assert_eq!(
            Into::<Vec<outcome::Outcome>>::into(Value::Sequence(sequence::Sequence {
                values: vec![]
            })),
            vec![outcome::Outcome {
                value: 0,
                weight: 1,
            }]
        );
    }
}
