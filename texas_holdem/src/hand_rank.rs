use playing_card::card::NonJokerCard;

mod hand_value;
pub use hand_value::*;

mod high_card;
mod pair;
mod two_pairs;
mod three_of_a_kind;
mod straight;
mod flush;
mod full_house;
mod four_of_a_kind;
mod straight_flush;
mod royal_flush;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub enum HandRank {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct BestFiveHand {
    cards: [NonJokerCard; 5],
    rank: HandRank,
}

impl BestFiveHand {
    pub fn cards(self) -> [NonJokerCard; 5] {
        self.cards
    }

    pub fn hand_rank(self) -> HandRank {
        self.rank
    }

    pub fn value(self) -> HandValue {
        let card_ranks = [
            self.cards[0].rank(),
            self.cards[1].rank(),
            self.cards[2].rank(),
            self.cards[3].rank(),
            self.cards[4].rank(),
        ];
        HandValue { hand_rank: self.hand_rank(), card_ranks }
    }
}