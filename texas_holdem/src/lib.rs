extern crate playing_card;

mod hand;

pub use hand::*;
use playing_card::cmp::CardComparer;
use playing_card::card::CardRank;

pub fn get_poker_card_comparer() -> CardComparer {
    CardComparer::new(CardRank::Ace)
}