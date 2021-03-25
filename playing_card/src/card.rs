mod suit;
mod rank;

use std::fmt;
pub use suit::*;
pub use rank::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Card {
    Joker,
    Other(NonJokerCard),
}

impl Card {
    pub fn new(suit: Suit, rank: CardRank) -> Self {
        Self::Other( NonJokerCard{ suit, rank })
    }

    pub fn all_cards_with_one_joker() -> Vec<Card>
    {
        let mut cards : Vec<Card> = NonJokerCard::all().iter().map(|c| Card::Other(*c)).collect();
        cards.push(Card::Joker);
        cards
    }
    
    pub fn all_cards_with_two_jokers() -> Vec<Card>
    {
        let mut cards : Vec<Card> = Self::all_cards_with_one_joker();
        cards.push(Card::Joker);
        cards
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct NonJokerCard { suit: Suit, rank: CardRank }

impl NonJokerCard {
    pub fn new(suit: Suit, rank: CardRank) -> Self {
        Self { suit, rank }
    }

    pub fn all() -> Vec<Self> {
        Suit::all().iter().flat_map(|suit| {
            CardRank::all().iter().map(|rank| {
                NonJokerCard{ suit: *suit, rank: *rank }
            }).collect::<Vec<NonJokerCard>>()
        }).collect()
    }

    pub fn suit(self) -> Suit {
        self.suit
    }

    pub fn rank(self) -> CardRank {
        self.rank
    }
}

impl fmt::Display for NonJokerCard {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.suit().to_char(), self.rank().to_char())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_non_joker_cards_are_52_cards() {
        assert_eq!(52, NonJokerCard::all().len());
    }

    #[test]
    fn all_non_joker_cards_are_all_different() {
        let cards = NonJokerCard::all();
        let card_count = cards.len();
        let unique_set : HashSet<NonJokerCard> = cards.into_iter().collect();
        assert_eq!(card_count, unique_set.len());
    }

    #[test]
    fn all_cards_with_one_joker_are_53_cards() {
        assert_eq!(53, Card::all_cards_with_one_joker().len());
    }

    #[test]
    fn all_cards_with_one_joker_are_all_different() {
        let cards = Card::all_cards_with_one_joker();
        let card_count = cards.len();
        let unique_set : HashSet<Card> = cards.into_iter().collect();
        assert_eq!(card_count, unique_set.len());
    }

    #[test]
    fn all_cards_with_two_jokers_are_54_cards() {
        assert_eq!(54, Card::all_cards_with_two_jokers().len());
    }

    #[test]
    fn all_cards_with_two_jokers_have_two_joker_cards() {
        let joker_count = Card::all_cards_with_two_jokers().into_iter()
                             .filter(|c| *c == Card::Joker).count();
        assert_eq!(2, joker_count);
    }
}
