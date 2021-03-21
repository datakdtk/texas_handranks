use super::card::CardRank;
use super::card::NonJokerCard;
use super::card::Suit;
use std::cmp::Ordering;
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct CardComparer { highest_rank: CardRank, suit_strengths: HashMap<Suit, u8> }

impl CardComparer {
    pub fn new(highest_rank: CardRank) -> Self {
        let default_order = [
            Suit::Spade,
            Suit::Heart,
            Suit::Diamond,
            Suit::Club,
        ];
        Self::new_with_suit_order(highest_rank, default_order)
            .expect("Something is wrong in default suit order")
    }

    /// Give suit_order in descending order. The former suit is greater.
    pub fn new_with_suit_order(highest_rank: CardRank, suit_order: [Suit;4]) -> Result<Self, String> {
        let mut suit_strengths = HashMap::new();
        for i in 0 ..= 3 {
            let old_value = suit_strengths.insert(suit_order[usize::from(i)], 4 - i);
            if let Some(x) = old_value {
                return Err(format!("{} is duplicated in suit order", x));
            }
        }
        Ok(Self {highest_rank, suit_strengths})
    }

    fn adjust_card_rank_value(&self, target: CardRank) -> u8 {
        if target.to_int() <= self.highest_rank.to_int() {
            target.to_int() + CardRank::King.to_int()
        } else {
            target.to_int()
        }
    }

    pub fn cmp_card_ranks(&self, a: CardRank, b: CardRank) -> Ordering {
        self.adjust_card_rank_value(a)
            .cmp(&self.adjust_card_rank_value(b))
    }

    pub fn cmp_suits(&self, a: Suit, b: Suit) -> Ordering {
        self.suit_strengths[&a].cmp(&self.suit_strengths[&b])
    }

    pub fn cmp_cards(&self, a: NonJokerCard, b: NonJokerCard ) -> Ordering {
        self.cmp_card_ranks(a.rank(), b.rank())
            .then(self.cmp_suits(a.suit(), b.suit()))
    }
}