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

impl Outputs {
    pub fn outcomes(&self) -> &Vec<Output> {
        &self.outputs
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    outcomes: Vec<Outcome>,
    name: Option<String>,
}

impl Output {
    pub fn new(outcomes: Vec<Outcome>, name: Option<String>) -> Self {
        Self { outcomes, name }
    }

    pub fn outcomes(&self) -> &Vec<Outcome> {
        &self.outcomes
    }
}
