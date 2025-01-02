use std::collections;
use std::iter;

use super::super::chance;

#[derive(Clone, Debug, PartialEq)]
pub(in crate::interpret) struct Dice {
    count: i32,
    die: Die,
}

impl Dice {
    pub(in crate::interpret) fn new(count: i32, die: Die) -> Self {
        Self { count, die }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(in crate::interpret) struct Die {
    chances: collections::BTreeMap<i32, chance::Chance>,
}

impl Die {
    pub(in crate::interpret) fn new(
        chances: impl iter::IntoIterator<Item = (i32, chance::Chance)>,
    ) -> Self {
        Self {
            chances: chances.into_iter().fold(
                collections::BTreeMap::new(),
                |mut acc, (value, chance)| {
                    acc.entry(value)
                        .and_modify(|prev| *prev += chance)
                        .or_insert(chance);
                    acc
                },
            ),
        }
    }
}
