extern crate playing_card;


pub mod hand_rank;
mod starting_hand;
mod total_hand;

pub use starting_hand::*;
pub use total_hand::*;

use playing_card::cmp::CardComparer;
use playing_card::card::CardRank;
use std::cmp;

pub fn get_poker_card_comparer() -> CardComparer {
    CardComparer::new(CardRank::Ace)
}

pub fn cmp_card_ranks(a: CardRank, b: CardRank) -> cmp::Ordering {
    let comparer = get_poker_card_comparer();
    comparer.cmp_card_ranks(a, b)
}