use crate::outcome::Outcome;

#[derive(Clone, Debug, PartialEq)]
pub struct Outputs {
    outputs: Vec<Output>,
}

impl Default for Outputs {
    fn default() -> Self {
        Self::new()
    }
}

impl Outputs {
    pub(super) fn new() -> Self {
        Self { outputs: vec![] }
    }

    pub(super) fn add_output(&mut self, output: Output) {
        self.outputs.push(output);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    outcomes: Vec<Outcome>,
    name: Option<String>,
}

impl Output {
    pub fn new(outcomes: Vec<Outcome>) -> Self {
        Self {
            outcomes,
            name: None,
        }
    }

    pub fn named(outcomes: Vec<Outcome>, name: String) -> Self {
        Self {
            outcomes,
            name: Some(name),
        }
    }

    pub fn outcomes(&self) -> &Vec<Outcome> {
        &self.outcomes
    }

    pub fn into_outcomes(self) -> Vec<Outcome> {
        self.outcomes
    }
}
