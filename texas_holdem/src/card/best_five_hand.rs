use super::{ HandRank, HandValue };
use playing_card::card::NonJokerCard;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct BestFiveHand {
    pub(super) cards: [NonJokerCard; 5],
    pub(super) rank: HandRank,
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