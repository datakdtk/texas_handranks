extern crate playing_card;

pub mod card;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub enum Phase {
    PreFlop,
    Flop,
    Turn,
    River,
}