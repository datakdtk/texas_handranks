use super::{ BestFiveHand, TotalHand};

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

impl TotalHand {
    pub fn find_best_five_hand(&self) -> Option<BestFiveHand> {
        royal_flush::try_to_build_from_total_hand(self)
            .or_else(|| straight_flush::try_to_build_from_total_hand(self))
            .or_else(|| four_of_a_kind::try_to_build_from_total_hand(self))
            .or_else(|| full_house::try_to_build_from_total_hand(self))
            .or_else(|| flush::try_to_build_from_total_hand(self))
            .or_else(|| straight::try_to_build_from_total_hand(self))
            .or_else(|| three_of_a_kind::try_to_build_from_total_hand(self))
            .or_else(|| two_pairs::try_to_build_from_total_hand(self))
            .or_else(|| pair::try_to_build_from_total_hand(self))
            .or_else(|| high_card::try_to_build_from_total_hand(self))
    }
}


#[cfg(test)]
mod test {
    use crate::card::HandRank;
    use super::*;
    use playing_card::card:: { CardRank, NonJokerCard, Suit };

    #[test]
    fn can_find_royal_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = hand.find_best_five_hand();
        assert_ne!(None, result);
        assert_eq!(HandRank::RoyalFlush, result.unwrap().hand_rank())
    }

    #[test]
    fn can_find_straight_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = hand.find_best_five_hand();
        assert_ne!(None, result);
        assert_eq!(HandRank::StraightFlush, result.unwrap().hand_rank())
    }

    #[test]
    fn can_find_four_of_a_kind() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
            NonJokerCard::new(Suit::Club, CardRank::new(13)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = hand.find_best_five_hand();
        assert_ne!(None, result);
        assert_eq!(HandRank::FourOfAKind, result.unwrap().hand_rank())
    }

    #[test]
    fn can_find_full_house() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
            NonJokerCard::new(Suit::Club, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = hand.find_best_five_hand();
        assert_ne!(None, result);
        assert_eq!(HandRank::FullHouse, result.unwrap().hand_rank())
    }

    #[test]
    fn can_find_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(6)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = hand.find_best_five_hand();
        assert_ne!(None, result);
        assert_eq!(HandRank::Flush, result.unwrap().hand_rank())
    }

    #[test]
    fn can_find_straight() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Spade, CardRank::new(12)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(11)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = hand.find_best_five_hand();
        assert_ne!(None, result);
        assert_eq!(HandRank::Straight, result.unwrap().hand_rank())
    }

    #[test]
    fn can_find_three_of_a_kind() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
            NonJokerCard::new(Suit::Club, CardRank::new(13)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = hand.find_best_five_hand();
        assert_ne!(None, result);
        assert_eq!(HandRank::ThreeOfAKind, result.unwrap().hand_rank())
    }

    #[test]
    fn can_find_two_pairs() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(9)),
            NonJokerCard::new(Suit::Club, CardRank::new(1)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = hand.find_best_five_hand();
        assert_ne!(None, result);
        assert_eq!(HandRank::TwoPairs, result.unwrap().hand_rank())
    }

    #[test]
    fn can_find_pair() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Club, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = hand.find_best_five_hand();
        assert_ne!(None, result);
        assert_eq!(HandRank::Pair, result.unwrap().hand_rank())
    }

    #[test]
    fn can_find_high_card() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Spade, CardRank::new(11)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Club, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = hand.find_best_five_hand();
        assert_ne!(None, result);
        assert_eq!(HandRank::HighCard, result.unwrap().hand_rank())
    }

    #[test]
    fn returns_none_when_4_or_less_cards() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Spade, CardRank::new(11)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = hand.find_best_five_hand();
        assert_eq!(None, result);
    }
}