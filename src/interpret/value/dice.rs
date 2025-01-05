use std::cmp;
use std::collections;
use std::iter;
use std::ops;

use crate::interpret::chance;

#[derive(Clone, Debug, PartialEq)]
pub(in crate::interpret) struct Dice {
    pub(in crate::interpret) count: i32,
    pub(in crate::interpret) die: Die,
}

impl Dice {
    pub(in crate::interpret) fn new(count: i32, die: Die) -> Self {
        Self { count, die }
    }
}

impl Dice {
    pub(in crate::interpret) fn flatten(self) -> Self {
        Self::new(1, self.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(in crate::interpret) struct Die {
    pub(in crate::interpret) chances: collections::BTreeMap<i32, chance::Chance>,
}

impl From<Dice> for Die {
    fn from(val: Dice) -> Self {
        if val.count == 0 {
            return Die::new(vec![]);
        }

        let flattened = iter::repeat_n(
            val.die,
            usize::try_from(val.count.abs()).expect("to be an absolute value"),
        )
        .reduce(|acc, curr| {
            Die::new(curr.chances.iter().flat_map(|(&curr_side, &curr_chance)| {
                acc.chances
                    .iter()
                    .map(|(&acc_side, &acc_chance)| {
                        (curr_side + acc_side, curr_chance * acc_chance)
                    })
                    .collect::<Vec<_>>()
            }))
        })
        .expect("dice count to be non-zero");

        if val.count < 0 {
            flattened * -1
        } else {
            flattened
        }
    }
}

macro_rules! die_side_arithmetic_impl {
    ($trait: ty, $fn: ident, $op: tt) => {
        impl $trait for Die {
            type Output = Self;

            fn $fn(self, rhs: i32) -> Self::Output {
                Self::Output::new(
                    self.chances
                        .into_iter()
                        .map(|(side, chance)| (side $op rhs, chance)),
                )
            }
        }
    };
}

die_side_arithmetic_impl!(ops::Add<i32>, add, +);
die_side_arithmetic_impl!(ops::Sub<i32>, sub, -);
die_side_arithmetic_impl!(ops::Mul<i32>, mul, *);
die_side_arithmetic_impl!(ops::Div<i32>, div, /);

impl Die {
    pub(in crate::interpret) fn new(
        chances: impl iter::IntoIterator<Item = (i32, chance::Chance)>,
    ) -> Self {
        Self {
            chances: chances
                .into_iter()
                .filter(|(_, chance)| chance.numerator != 0)
                .fold(collections::BTreeMap::new(), |mut acc, (value, chance)| {
                    acc.entry(value)
                        .and_modify(|prev| *prev = *prev + chance)
                        .or_insert(chance);
                    acc
                }),
        }
    }

    pub(in crate::interpret) fn new_regular(largest_side: i32) -> Self {
        Self::new(
            match largest_side.cmp(&0) {
                cmp::Ordering::Less => largest_side..0,
                cmp::Ordering::Equal => 0..0,
                cmp::Ordering::Greater => 1..(largest_side + 1),
            }
            .map(|side| {
                (
                    side,
                    chance::Chance::new(
                        1,
                        usize::try_from(largest_side.abs()).expect("to be an absolute value"),
                    ),
                )
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_die_new() {
        assert_eq!(
            Die::new(vec![
                (1, chance::Chance::new(1, 4)),
                (2, chance::Chance::new(1, 4)),
                (3, chance::Chance::new(1, 4)),
                (4, chance::Chance::new(1, 4)),
            ]),
            Die::new_regular(4)
        );
    }

    #[test]
    fn test_die_new_combines_same_sides() {
        assert_eq!(
            Die::new(vec![
                (1, chance::Chance::new(4, 32)),
                (2, chance::Chance::new(2, 16)),
                (3, chance::Chance::new(1, 8)),
                (4, chance::Chance::new(1, 4)),
                (3, chance::Chance::new(1, 8)),
                (2, chance::Chance::new(2, 16)),
                (1, chance::Chance::new(4, 32)),
            ]),
            Die::new_regular(4)
        );
    }

    #[test]
    fn test_die_new_filters_out_zero_chances() {
        assert_eq!(
            Die::new(vec![
                (1, chance::Chance::new(1, 4)),
                (2, chance::Chance::new(1, 4)),
                (3, chance::Chance::new(1, 4)),
                (4, chance::Chance::new(1, 4)),
                (5, chance::Chance::new(0, 4)),
            ]),
            Die::new_regular(4)
        );
    }

    #[test]
    fn test_die_from_dice() {
        let mut chances = collections::BTreeMap::new();
        chances.insert(2, chance::Chance::new(1, 16));
        chances.insert(3, chance::Chance::new(2, 16));
        chances.insert(4, chance::Chance::new(3, 16));
        chances.insert(5, chance::Chance::new(4, 16));
        chances.insert(6, chance::Chance::new(3, 16));
        chances.insert(7, chance::Chance::new(2, 16));
        chances.insert(8, chance::Chance::new(1, 16));
        let die = Die { chances };

        assert_eq!(Into::<Die>::into(Dice::new(2, Die::new_regular(4))), die);
    }

    #[test]
    fn test_flatten_dice() {
        let mut chances = collections::BTreeMap::new();
        chances.insert(2, chance::Chance::new(1, 16));
        chances.insert(3, chance::Chance::new(2, 16));
        chances.insert(4, chance::Chance::new(3, 16));
        chances.insert(5, chance::Chance::new(4, 16));
        chances.insert(6, chance::Chance::new(3, 16));
        chances.insert(7, chance::Chance::new(2, 16));
        chances.insert(8, chance::Chance::new(1, 16));
        let die = Die { chances };

        assert_eq!(
            Dice::new(2, Die::new_regular(4)).flatten(),
            Dice::new(1, die)
        );
    }

    #[test]
    fn test_die_add_scalar() {
        assert_eq!(Die::new_regular(4) * 1, Die::new_regular(4));

        assert_eq!(
            Die::new_regular(4) * 2,
            Die::new(vec![
                (2, chance::Chance::new(1, 4)),
                (4, chance::Chance::new(1, 4)),
                (6, chance::Chance::new(1, 4)),
                (8, chance::Chance::new(1, 4)),
            ])
        );

        assert_eq!(
            Die::new_regular(4) * -1,
            Die::new(vec![
                (-1, chance::Chance::new(1, 4)),
                (-2, chance::Chance::new(1, 4)),
                (-3, chance::Chance::new(1, 4)),
                (-4, chance::Chance::new(1, 4)),
            ])
        );

        assert_eq!(
            Die::new_regular(4) * 0,
            Die::new(vec![(0, chance::Chance::new(1, 1))])
        );
    }

    #[test]
    fn test_die_mul_scalar() {
        assert_eq!(Die::new_regular(4) * 1, Die::new_regular(4));

        assert_eq!(
            Die::new_regular(4) * 2,
            Die::new(vec![
                (2, chance::Chance::new(1, 4)),
                (4, chance::Chance::new(1, 4)),
                (6, chance::Chance::new(1, 4)),
                (8, chance::Chance::new(1, 4)),
            ])
        );

        assert_eq!(
            Die::new_regular(4) * -1,
            Die::new(vec![
                (-1, chance::Chance::new(1, 4)),
                (-2, chance::Chance::new(1, 4)),
                (-3, chance::Chance::new(1, 4)),
                (-4, chance::Chance::new(1, 4)),
            ])
        );

        assert_eq!(
            Die::new_regular(4) * 0,
            Die::new(vec![(0, chance::Chance::new(1, 1))])
        );
    }

    #[test]
    fn test_die_div_scalar() {
        assert_eq!(Die::new_regular(4) / 1, Die::new_regular(4));

        assert_eq!(
            Die::new_regular(4) / 2,
            Die::new(vec![
                (0, chance::Chance::new(1, 4)),
                (1, chance::Chance::new(1, 2)),
                (2, chance::Chance::new(1, 4)),
            ])
        );

        assert_eq!(
            Die::new_regular(4) / -1,
            Die::new(vec![
                (-1, chance::Chance::new(1, 4)),
                (-2, chance::Chance::new(1, 4)),
                (-3, chance::Chance::new(1, 4)),
                (-4, chance::Chance::new(1, 4)),
            ])
        );
    }
}
