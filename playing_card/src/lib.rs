extern crate rand;

mod suit;
mod rank;
pub mod deck;

pub use suit::*;
pub use rank::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Card {
    Joker,
    Other(NonJokerCard),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct NonJokerCard { pub suit: Suit, pub rank: CardRank }