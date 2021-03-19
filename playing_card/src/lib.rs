mod suit;
mod rank;

pub use suit::*;
pub use rank::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Card {
    Joker,
    Other(NonJokerCard),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct NonJokerCard { pub suit: Suit, pub rank: CardRank }