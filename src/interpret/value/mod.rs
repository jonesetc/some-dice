mod dice;
mod number;
mod sequence;

#[derive(Clone, Debug, PartialEq)]
pub(super) enum Value {
    Number(number::Number),
    Dice(dice::Dice),
    Sequence(sequence::Sequence),
}
