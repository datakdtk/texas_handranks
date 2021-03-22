use playing_card::card::CardRank;
use playing_card::card::NonJokerCard;
use playing_card::card::Suit;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Hand(NonJokerCard, NonJokerCard);

impl Hand {
    pub fn new(a: NonJokerCard, b: NonJokerCard) -> Self {
        let comparer = super::get_poker_card_comparer();
        if comparer.cmp_cards(&a, &b) == Ordering::Greater {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }

    pub fn higher_card(self) -> NonJokerCard {
        self.0
    }

    pub fn lower_card(self) -> NonJokerCard {
        self.1
    }

    pub fn both_cards(self) -> [NonJokerCard;2] {
        [self.0, self.1]
    }

    pub fn has_rank_of(self, rank: CardRank) -> bool {
        self.0.rank() == rank || self.1.rank() == rank
    }

    pub fn has_any_rank_of(self, ranks: &[CardRank]) -> bool {
        ranks.iter().any(|rank| self.has_rank_of(*rank))
    }

    pub fn is_suited(self) -> bool {
        self.0.suit() == self.1.suit()
    }

    pub fn has_suit_of(self, suit: Suit) -> bool {
        self.0.suit() == suit || self.1.suit() == suit
    }

    pub fn is_suited_in(self, suit: Suit) -> bool {
        self.is_suited() && self.has_suit_of(suit)
    }

    pub fn is_pair(self) -> bool {
        self.0.rank() == self.1.rank()
    }

    pub fn is_pair_of(self, rank: CardRank) -> bool {
        self.is_pair() && self.has_rank_of(rank)
    }

    pub fn is_connector(self) -> bool {
        match self.0.rank() {
            CardRank::Ace => self.1.rank().to_int() == 13 || self.1.rank().to_int() == 2,
            _ => self.1.rank().to_int() == self.0.rank().to_int() - 1,
        }
    }

    pub fn is_suited_connector(self) -> bool {
        self.is_suited() && self.is_connector()
    }

    pub fn is_one_gapper(self) -> bool {
        match self.0.rank() {
            CardRank::Ace => self.1.rank().to_int() == 12 || self.1.rank().to_int() == 3,
            _ => self.1.rank().to_int() == self.0.rank().to_int() - 2,
        }
    }

    pub fn is_suited_one_gapper(self) -> bool {
        self.is_suited() && self.is_one_gapper()
    }

    pub fn summary(self) -> String {
        let suffix = if self.is_pair() {
            ""
        } else if self.is_suited() {
            "s"
        } else {
            "o"
        };
        String::from(format!("{}{}{}", self.0.rank().to_char(), self.1.rank().to_char(), suffix))
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.0, self.1)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn higher_card_is_first_item_when_first_item_is_higher_than_second() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert_eq!(a, hand.higher_card());
    }

    #[test]
    fn lower_card_is_second_item_when_first_item_is_higher_than_second() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert_eq!(b, hand.lower_card());
    }

    #[test]
    fn lower_card_is_first_item_when_first_item_is_lower_than_second() {
        let a = NonJokerCard::new(Suit::Club, CardRank::Jack);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert_eq!(a, hand.lower_card());
    }

    #[test]
    fn higher_card_is_second_item_when_first_item_is_lower_than_second() {
        let a = NonJokerCard::new(Suit::Club, CardRank::Jack);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert_eq!(b, hand.higher_card());
    }

    #[test]
    fn higher_card_comes_first_in_both_cards_when_first_item_is_higher_than_second() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert_eq!(hand.higher_card(), hand.both_cards()[0]);
        assert_eq!(hand.lower_card(), hand.both_cards()[1]);
    }

    #[test]
    fn higher_card_comes_first_in_both_cards_when_first_item_is_lower_than_second() {
        let a = NonJokerCard::new(Suit::Club, CardRank::Jack);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert_eq!(hand.higher_card(), hand.both_cards()[0]);
        assert_eq!(hand.lower_card(), hand.both_cards()[1]);
    }

    #[test]
    fn has_rank_of_matches_higher_card() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.has_rank_of(CardRank::King))
    }

    #[test]
    fn has_rank_of_matches_lower_card() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.has_rank_of(CardRank::Queen))
    }

    #[test]
    fn has_rank_of_does_not_matches() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(!hand.has_rank_of(CardRank::Jack))
    }

    #[test]
    fn has_any_rank_of_matches_higher_card() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.has_any_rank_of(&[CardRank::Ace, CardRank::King]))
    }

    #[test]
    fn has_any_rank_of_matches_lower_card() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.has_any_rank_of(&[CardRank::Ace, CardRank::Queen]))
    }

    #[test]
    fn has_any_rank_of_does_not_match() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(!hand.has_any_rank_of(&[CardRank::Ace, CardRank::Jack]))
    }

    #[test]
    fn is_suited() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.is_suited())
    }
 
    #[test]
    fn is_not_suited() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(!hand.is_suited())
    }
 
    #[test]
    fn has_suit_of_matches_higher_card() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.has_suit_of(Suit::Heart))
    }
 
    #[test]
    fn has_suit_of_matches_lower_card() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.has_suit_of(Suit::Club))
    }
 
    #[test]
    fn has_suit_of_does_not_match() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(!hand.has_suit_of(Suit::Spade))
    }

    #[test]
    fn is_suited_in_returns_true_when_suited_and_suit_matches() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.is_suited_in(Suit::Club))
    }

    #[test]
    fn is_suited_in_returns_false_when_suited_but_suit_does_not_match() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Club, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.is_suited());
        assert!(!hand.is_suited_in(Suit::Heart))
    }

    #[test]
    fn is_suited_in_returns_false_when_suit_matches_but_not_suited() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.has_suit_of(Suit::Club));
        assert!(!hand.is_suited_in(Suit::Club))
    }

    #[test]
    fn is_pair_returns_true_when_same_ranks() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::King);
        let hand = Hand::new(a, b);
        assert!(hand.is_pair())
    }

    #[test]
    fn is_pair_returns_false_when_different_ranks() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Ace);
        let hand = Hand::new(a, b);
        assert!(!hand.is_pair())
    }

    #[test]
    fn is_pair_of_returns_true_when_same_ranks_and_rank_matches() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::King);
        let hand = Hand::new(a, b);
        assert!(hand.is_pair_of(CardRank::King))
    }

    #[test]
    fn is_pair_of_returns_false_when_same_ranks_but_rank_does_not_match() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::King);
        let hand = Hand::new(a, b);
        assert!(hand.is_pair());
        assert!(!hand.is_pair_of(CardRank::Queen))
    }

    #[test]
    fn is_pair_of_returns_false_when_rank_matches_but_not_same_ranks() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.has_rank_of(CardRank::Queen));
        assert!(!hand.is_pair_of(CardRank::Queen))
    }

    #[test]
    fn king_and_queen_is_connector() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.is_connector())
    }

    #[test]
    fn king_and_ace_is_connector() {
        let a = NonJokerCard::new(Suit::Club, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Ace);
        let hand = Hand::new(a, b);
        assert!(hand.is_connector())
    }

    #[test]
    fn ace_and_2_is_connector() {
        let a = NonJokerCard::new(Suit::Club, CardRank::new(2).unwrap());
        let b = NonJokerCard::new(Suit::Heart, CardRank::Ace);
        let hand = Hand::new(a, b);
        assert!(hand.is_connector())
    }

    #[test]
    fn ace_and_queen_is_not_connector() {
        let a = NonJokerCard::new(Suit::Club, CardRank::Ace);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(!hand.is_connector())
    }

    #[test]
    fn ace_and_ace_is_not_connector() {
        let a = NonJokerCard::new(Suit::Club, CardRank::Ace);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Ace);
        let hand = Hand::new(a, b);
        assert!(!hand.is_connector())
    }

    #[test]
    fn is_suited_connector_returns_true_when_it_is_suited_and_is_connector() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.is_suited_connector())
    }

    #[test]
    fn is_suited_connector_returns_false_when_it_is_connector_but_not_suited() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::King);
        let b = NonJokerCard::new(Suit::Spade, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.is_connector());
        assert!(!hand.is_suited_connector())
    }

    #[test]
    fn is_suited_connector_returns_false_when_it_is_suited_but_not_connector() {
        let a = NonJokerCard::new(Suit::Spade, CardRank::King);
        let b = NonJokerCard::new(Suit::Spade, CardRank::Jack);
        let hand = Hand::new(a, b);
        assert!(hand.is_suited());
        assert!(!hand.is_suited_connector())
    }

    #[test]
    fn king_and_jack_is_one_gapper() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Jack);
        let hand = Hand::new(a, b);
        assert!(hand.is_one_gapper())
    }

    #[test]
    fn king_and_queen_is_not_one_gapper() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(!hand.is_one_gapper())
    }

    #[test]
    fn king_and_king_is_not_one_gapper() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::King);
        let hand = Hand::new(a, b);
        assert!(!hand.is_one_gapper())
    }

    #[test]
    fn king_and_2_is_not_one_gapper() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::new(2).unwrap());
        let hand = Hand::new(a, b);
        assert!(!hand.is_one_gapper())
    }

    #[test]
    fn ace_and_3_is_one_gapper() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::Ace);
        let b = NonJokerCard::new(Suit::Heart, CardRank::new(3).unwrap());
        let hand = Hand::new(a, b);
        assert!(hand.is_one_gapper())
    }

    #[test]
    fn ace_and_queen_is_one_gapper() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::Ace);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.is_one_gapper())
    }

    #[test]
    fn is_suited_one_gapper_returns_true_when_it_is_suited_and_is_one_gapper() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::King);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Jack);
        let hand = Hand::new(a, b);
        assert!(hand.is_suited_one_gapper())
    }

    #[test]
    fn is_suited_one_gapper_returns_false_when_it_is_one_gapper_but_not_suited() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::King);
        let b = NonJokerCard::new(Suit::Spade, CardRank::Jack);
        let hand = Hand::new(a, b);
        assert!(hand.is_one_gapper());
        assert!(!hand.is_suited_one_gapper())
    }

    #[test]
    fn is_suited_one_gapper_returns_false_when_it_is_suited_but_not_one_gapper() {
        let a = NonJokerCard::new(Suit::Spade, CardRank::King);
        let b = NonJokerCard::new(Suit::Spade, CardRank::Queen);
        let hand = Hand::new(a, b);
        assert!(hand.is_suited());
        assert!(!hand.is_suited_one_gapper())
    }

    #[test]
    fn summary_of_pair_has_no_suffix() {
        let a = NonJokerCard::new(Suit::Spade, CardRank::Ace);
        let b = NonJokerCard::new(Suit::Club, CardRank::Ace);
        let hand = Hand::new(a, b);
        assert_eq!("AA", hand.summary())
    }

    #[test]
    fn summary_of_suited_hand_has_suffix_s() {
        let a = NonJokerCard::new(Suit::Spade, CardRank::King);
        let b = NonJokerCard::new(Suit::Spade, CardRank::Ace);
        let hand = Hand::new(a, b);
        assert_eq!("AKs", hand.summary())
    }
 
    #[test]
    fn summary_of_offsuit_hand_has_suffix_o() {
        let a = NonJokerCard::new(Suit::Heart, CardRank::King);
        let b = NonJokerCard::new(Suit::Spade, CardRank::Ace);
        let hand = Hand::new(a, b);
        assert_eq!("AKo", hand.summary())
    }
}
