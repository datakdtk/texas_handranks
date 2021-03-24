extern crate playing_card;


pub mod hand_rank;
mod starting_hand;
mod total_hand;

pub use starting_hand::*;
pub use total_hand::*;

use playing_card::cmp::CardComparer;
use playing_card::card::{ CardRank, NonJokerCard };
use std::cmp;

fn get_poker_card_comparer() -> CardComparer {
    CardComparer::new(CardRank::Ace)
}

pub fn cmp_card_ranks(a: CardRank, b: CardRank) -> cmp::Ordering {
    let comparer = get_poker_card_comparer();
    comparer.cmp_card_ranks(a, b)
}

pub fn cmp_cards(a: NonJokerCard, b: NonJokerCard) -> cmp::Ordering {
    let comparer = get_poker_card_comparer();
    comparer.cmp_cards(a, b)
}