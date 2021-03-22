use super::card::CardRank;
use super::card::NonJokerCard;
use super::card::Suit;
use std::cmp::Ordering;
use std::collections::HashMap;


#[derive(Debug, Clone)]
/// CardComparer compares two card ranks, suits, or non-joker cards in a flexible scale.
pub struct CardComparer { highest_rank: CardRank, suit_strengths: HashMap<Suit, u8> }

impl CardComparer {
    /// Create new object with default suit order.
    /// The default order of suits may change in the future.
    /// It is recommended to use new_with_suit_order instead if your logic depends on specific order.
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

    /// Create new object with specific suit order.
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

    fn adjust_card_rank_value(&self, target: &CardRank) -> u8 {
        if target.to_int() <= self.highest_rank.to_int() {
            target.to_int() + CardRank::King.to_int()
        } else {
            target.to_int()
        }
    }

    /// Returns Ordering::Greater if former item is greater.
    pub fn cmp_card_ranks(&self, a: &CardRank, b: &CardRank) -> Ordering {
        self.adjust_card_rank_value(a)
            .cmp(&self.adjust_card_rank_value(b))
    }

    /// Returns Ordering::Greater if former item is greater.
    pub fn cmp_suits(&self, a: &Suit, b: &Suit) -> Ordering {
        self.suit_strengths[a].cmp(&self.suit_strengths[b])
    }

    /// Compare cards by their ranks at first, then by their suits.
    /// Returns Ordering::Greater if former item is greater.
    pub fn cmp_cards(&self, a: &NonJokerCard, b: &NonJokerCard ) -> Ordering {
        self.cmp_card_ranks(&a.rank(), &b.rank())
            .then(self.cmp_suits(&a.suit(), &b.suit()))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn get_all_ranks() -> Vec<CardRank> {
        vec![
            CardRank::King,
            CardRank::Queen,
            CardRank::Jack,
            CardRank::new(10).unwrap(),
            CardRank::new(9).unwrap(),
            CardRank::new(8).unwrap(),
            CardRank::new(7).unwrap(),
            CardRank::new(6).unwrap(),
            CardRank::new(5).unwrap(),
            CardRank::new(4).unwrap(),
            CardRank::new(3).unwrap(),
            CardRank::new(2).unwrap(),
            CardRank::Ace,
        ]
    }

    #[test]
    fn test_rank_order_when_king_is_highest() {
        let comparer = CardComparer::new(CardRank::King);
        let mut ranks = get_all_ranks();
        let expected = vec![
            CardRank::Ace,
            CardRank::new(2).unwrap(),
            CardRank::new(3).unwrap(),
            CardRank::new(4).unwrap(),
            CardRank::new(5).unwrap(),
            CardRank::new(6).unwrap(),
            CardRank::new(7).unwrap(),
            CardRank::new(8).unwrap(),
            CardRank::new(9).unwrap(),
            CardRank::new(10).unwrap(),
            CardRank::Jack,
            CardRank::Queen,
            CardRank::King,
        ];
        ranks.sort_by(|a, b| comparer.cmp_card_ranks(a, b) );
        assert_eq!(expected, ranks);
    }

    #[test]
    fn test_rank_order_when_ace_is_highest() {
        let comparer = CardComparer::new(CardRank::Ace);
        let mut ranks = get_all_ranks();
        let expected = vec![
            CardRank::new(2).unwrap(),
            CardRank::new(3).unwrap(),
            CardRank::new(4).unwrap(),
            CardRank::new(5).unwrap(),
            CardRank::new(6).unwrap(),
            CardRank::new(7).unwrap(),
            CardRank::new(8).unwrap(),
            CardRank::new(9).unwrap(),
            CardRank::new(10).unwrap(),
            CardRank::Jack,
            CardRank::Queen,
            CardRank::King,
            CardRank::Ace,
        ];
        ranks.sort_by(|a, b| comparer.cmp_card_ranks(a, b) );
        assert_eq!(expected, ranks);
    }

    #[test]
    fn test_rank_order_when_2_is_highest() {
        let comparer = CardComparer::new(CardRank::new(2).unwrap());
        let mut ranks = get_all_ranks();
        let expected = vec![
            CardRank::new(3).unwrap(),
            CardRank::new(4).unwrap(),
            CardRank::new(5).unwrap(),
            CardRank::new(6).unwrap(),
            CardRank::new(7).unwrap(),
            CardRank::new(8).unwrap(),
            CardRank::new(9).unwrap(),
            CardRank::new(10).unwrap(),
            CardRank::Jack,
            CardRank::Queen,
            CardRank::King,
            CardRank::Ace,
            CardRank::new(2).unwrap(),
        ];
        ranks.sort_by(|a, b| comparer.cmp_card_ranks(a, b) );
        assert_eq!(expected, ranks);
    }

    #[test]
    fn it_can_compare_suits_in_customized_order_1() {
        let suits_order = [
            Suit::Heart,
            Suit::Club,
            Suit::Spade,
            Suit::Diamond,
        ];
        let comparer = CardComparer::new_with_suit_order(CardRank::King, suits_order).expect("constructor error");
        let mut target = vec![
            Suit::Spade,
            Suit::Diamond,
            Suit::Heart,
            Suit::Club,
        ];
        target.sort_by(|a, b| comparer.cmp_suits(a, b).reverse()); // must reverse because given order is descending
        let expected: Vec<Suit> = suits_order.iter().map(|s| *s).collect();
        assert_eq!(expected, target);
    }

    #[test]
    fn it_can_compare_suits_in_customized_order_2() {
        // in another order
        let suits_order = [
            Suit::Spade,
            Suit::Club,
            Suit::Heart,
            Suit::Diamond,
        ];
        let comparer = CardComparer::new_with_suit_order(CardRank::King, suits_order).expect("constructor error");
        let mut target = vec![
            Suit::Spade,
            Suit::Diamond,
            Suit::Heart,
            Suit::Club,
        ];
        target.sort_by(|a, b| comparer.cmp_suits(a, b).reverse()); // must reverse because given order is descending
        let expected: Vec<Suit> = suits_order.iter().map(|s| *s).collect();
        assert_eq!(expected, target);
    }

    #[test]
    fn duplicate_suits_order_causes_error() {
        // in another order
        let suits_order = [
            Suit::Spade,
            Suit::Spade,
            Suit::Heart,
            Suit::Diamond,
        ];
        let result = CardComparer::new_with_suit_order(CardRank::King, suits_order);
        if let Ok(_) = result {
            panic!("Error is expected but not occurred")
        }
    }

    #[test]
    fn cards_are_compared_by_rank_at_first() {
        let suits_order = [
            Suit::Heart,
            Suit::Club,
            Suit::Spade,
            Suit::Diamond,
        ];
        let comparer = CardComparer::new_with_suit_order(CardRank::Ace, suits_order).expect("constructor error");
        let a = NonJokerCard::new(Suit::Club, CardRank::Ace);
        let b = NonJokerCard::new(Suit::Heart, CardRank::King);
        assert_eq!(Ordering::Greater, comparer.cmp_cards(&a, &b))
    }

    #[test]
    fn same_rank_cards_are_compared_by_suit() {
        let suits_order = [
            Suit::Heart,
            Suit::Club,
            Suit::Spade,
            Suit::Diamond,
        ];
        let comparer = CardComparer::new_with_suit_order(CardRank::Ace, suits_order).expect("constructor error");
        let a = NonJokerCard::new(Suit::Club, CardRank::Ace);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Ace);
        assert_eq!(Ordering::Less, comparer.cmp_cards(&a, &b))
    }
 
    #[test]
    fn same_cards_are_equal() {
        let suits_order = [
            Suit::Heart,
            Suit::Club,
            Suit::Spade,
            Suit::Diamond,
        ];
        let comparer = CardComparer::new_with_suit_order(CardRank::Ace, suits_order).expect("constructor error");
        let a = NonJokerCard::new(Suit::Heart, CardRank::Ace);
        let b = NonJokerCard::new(Suit::Heart, CardRank::Ace);
        assert_eq!(Ordering::Equal, comparer.cmp_cards(&a, &b))
    }
}