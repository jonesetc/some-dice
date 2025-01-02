use std::collections;

use super::value;

#[derive(Clone, Debug, PartialEq, Default)]
pub(super) struct Environment {
    values: collections::HashMap<String, value::Value>,
}

impl Environment {
    pub(super) fn new() -> Self {
        Default::default()
    }

    pub(super) fn get_value(&self, key: &str) -> Option<&value::Value> {
        self.values.get(key)
    }
}
