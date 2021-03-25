mod best_five_hand;
mod board;
mod hand_value;
mod rank;
mod starting_hand;
mod total_hand;

pub use best_five_hand::BestFiveHand;
pub use board::Board;
pub use hand_value::HandValue;
pub use rank::HandRank;
pub use starting_hand::StartingHand;
pub use total_hand::TotalHand;

use playing_card::cmp::CardComparer;
use playing_card::card::{ CardRank, NonJokerCard };
use std::cmp::Ordering;

fn get_poker_card_comparer() -> CardComparer {
    CardComparer::new(CardRank::Ace)
}

pub fn cmp_card_ranks(a: CardRank, b: CardRank) -> Ordering {
    let comparer = get_poker_card_comparer();
    comparer.cmp_card_ranks(a, b)
}

pub fn cmp_cards(a: NonJokerCard, b: NonJokerCard) -> Ordering {
    let comparer = get_poker_card_comparer();
    comparer.cmp_cards(a, b)
}