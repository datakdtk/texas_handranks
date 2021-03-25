use crate::TotalHand;
use super::{ BestFiveHand, HandRank };
use playing_card::card:: { CardRank, NonJokerCard };


pub(super) fn try_to_build_from_total_hand(hand: &TotalHand) -> Option<BestFiveHand> {
    let maybe_suit_of_flush = hand.suit_of_flush();
    let head_ranks_of_straight = hand.head_ranks_of_straight();
    if head_ranks_of_straight.is_empty() || maybe_suit_of_flush.is_none() {
        return None;
    }
    let suit = maybe_suit_of_flush.unwrap();


    let mut card_vec: Vec<Option<&NonJokerCard>> = Vec::new();
    for head in head_ranks_of_straight {
        let head_rank_int = if head == CardRank::Ace { 14 } else { head.to_int() };
        let ranks = [
            head,
            CardRank::new(head_rank_int - 1),
            CardRank::new(head_rank_int - 2),
            CardRank::new(head_rank_int - 3),
            CardRank::new(head_rank_int - 4),
        ];
        card_vec = ranks.iter().map(|r| {
            hand.cards().iter().filter(|c| c.rank() == *r && c.suit() == suit).next()
        }).collect();
        if card_vec.iter().all(|opt| opt.is_some()) {
            break;
        }
    }
    if card_vec.iter().any(|opt| opt.is_none()) {
        return None;
    }

    assert_eq!(5, card_vec.len());

    let cards = [
        *card_vec[0].unwrap(),
        *card_vec[1].unwrap(),
        *card_vec[2].unwrap(),
        *card_vec[3].unwrap(),
        *card_vec[4].unwrap(),
    ];
    Some(BestFiveHand { cards, rank: HandRank::StraightFlush })
}


#[cfg(test)]
mod test {
    use super::*;
    use playing_card::card:: { CardRank, NonJokerCard, Suit };

    #[test]
    fn returns_none_when_straight_but_not_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_none_when_flush_but_not_straight() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_none_when_straight_and_flush_but_not_straight_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Spade, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_some_when_straight_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn returns_some_when_straight_has_duplicate_rank() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Spade, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn result_contains_ranks_of_straight_flush_when_higher_straight_is_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }

        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(13),
            CardRank::new(12),
            CardRank::new(11),
            CardRank::new(10),
            CardRank::new(9),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn result_contains_ranks_of_straight_flush_when_lower_straight_is_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }

        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(12),
            CardRank::new(11),
            CardRank::new(10),
            CardRank::new(9),
            CardRank::new(8),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn result_contains_ranks_of_straight_flush_when_middle_straight_is_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }

        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(12),
            CardRank::new(11),
            CardRank::new(10),
            CardRank::new(9),
            CardRank::new(8),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn result_contains_ranks_of_highest_straight_flush_when_same_suit_sequence_is_longer_than_5() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }

        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(13),
            CardRank::new(12),
            CardRank::new(11),
            CardRank::new(10),
            CardRank::new(9),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn hand_rank_of_result_is_straight_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        assert_eq!(HandRank::StraightFlush, result.unwrap().value().hand_rank)
    }
}