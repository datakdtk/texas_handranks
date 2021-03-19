mod suit;
mod rank;

pub use suit::*;
pub use rank::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Card {
    Joker,
    Other(NonJokerCard),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct NonJokerCard { suit: Suit, rank: CardRank }

impl NonJokerCard {
    pub fn suit(self) -> Suit {
        self.suit
    }

    pub fn rank(self) -> CardRank {
        self.rank
    }
}

pub fn all_non_joker_card() -> Vec<NonJokerCard> {
    Suit::all().iter().flat_map(|suit| {
        CardRank::all().iter().map(|rank| {
            NonJokerCard{ suit: *suit, rank: *rank }
        }).collect::<Vec<NonJokerCard>>()
    }).collect()
}

pub fn all_cards_with_one_joker() -> Vec<Card>
{
    let mut cards : Vec<Card> = all_non_joker_card().iter().map(|c| Card::Other(*c)).collect();
    cards.push(Card::Joker);
    cards
}

pub fn all_cards_with_two_jokers() -> Vec<Card>
{
    let mut cards : Vec<Card> = all_cards_with_one_joker();
    cards.push(Card::Joker);
    cards
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_non_joker_cards_are_52_cards() {
        assert_eq!(52, all_non_joker_card().len());
    }

    #[test]
    fn all_non_joker_cards_are_all_different() {
        let cards = all_non_joker_card();
        let card_count = cards.len();
        let unique_set : HashSet<NonJokerCard> = cards.into_iter().collect();
        assert_eq!(card_count, unique_set.len());
    }

    #[test]
    fn all_cards_with_one_joker_are_53_cards() {
        assert_eq!(53, all_cards_with_one_joker().len());
    }

    #[test]
    fn all_cards_with_one_joker_are_all_different() {
        let cards = all_cards_with_one_joker();
        let card_count = cards.len();
        let unique_set : HashSet<Card> = cards.into_iter().collect();
        assert_eq!(card_count, unique_set.len());
    }

    #[test]
    fn all_cards_with_two_jokers_are_54_cards() {
        assert_eq!(54, all_cards_with_two_jokers().len());
    }

    #[test]
    fn all_cards_with_two_jokers_have_two_joker_cards() {
        let joker_count = all_cards_with_two_jokers().into_iter().filter(|c| *c == Card::Joker).count();
        assert_eq!(2, joker_count);
    }
}
