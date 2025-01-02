#[derive(Clone, Debug, PartialEq)]
pub(super) struct Configuration {
    pub(super) position_order: PositionOrder,
    pub(super) maximum_function_depth: u8,
    pub(super) explode_depth: u8,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration::new()
    }
}

impl Configuration {
    pub(super) fn new() -> Self {
        Configuration {
            position_order: PositionOrder::HighestFirst,
            maximum_function_depth: 10,
            explode_depth: 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum PositionOrder {
    HighestFirst,
    LowestFirst,
}
