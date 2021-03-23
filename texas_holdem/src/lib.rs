extern crate playing_card;

pub mod hand_rank;
mod starting_hand;
pub use starting_hand::StartingHand;

use playing_card::cmp::CardComparer;
use playing_card::card::CardRank;

pub fn get_poker_card_comparer() -> CardComparer {
    CardComparer::new(CardRank::Ace)
}