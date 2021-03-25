use crate::TotalHand;
use super::{ BestFiveHand, HandRank, straight_flush};
use playing_card::card::CardRank;


pub(super) fn try_to_build_from_total_hand(hand: &TotalHand) -> Option<BestFiveHand> {
    let maybe_straight_flush_hand = straight_flush::try_to_build_from_total_hand(hand);
    if maybe_straight_flush_hand.is_none() {
        return None;
    }
    let straight_flush_hand = maybe_straight_flush_hand.unwrap(); 
    let head_rank = straight_flush_hand.cards()[0].rank();
    if head_rank != CardRank::Ace {
        return None;
    }
    Some(BestFiveHand { cards: straight_flush_hand.cards(), rank: HandRank::RoyalFlush })
}


#[cfg(test)]
mod test {
    use super::*;
    use playing_card::card:: { CardRank, NonJokerCard, Suit };

    #[test]
    fn returns_none_when_ace_high_straight_but_not_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(1)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }
 
    #[test]
    fn returns_none_when_straight_flush_but_that_is_not_ace_high() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(1)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }
 
    #[test]
    fn returns_none_when_5_to_ace_straight_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(1)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_some_when_ace_high_straight_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn returns_some_when_sequence_is_longer_than_5() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Heart, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn ranks_of_result_value_are_ace_to_10() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }

        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(1),
            CardRank::new(13),
            CardRank::new(12),
            CardRank::new(11),
            CardRank::new(10),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn hand_rank_of_result_is_royal_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        assert_eq!(HandRank::RoyalFlush, result.unwrap().value().hand_rank)
    }
}