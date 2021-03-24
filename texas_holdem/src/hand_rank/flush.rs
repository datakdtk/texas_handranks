use crate::TotalHand;
use super::{ BestFiveHand, HandRank };
use playing_card::card::NonJokerCard;


pub(super) fn try_to_build_from_total_hand(hand: TotalHand) -> Option<BestFiveHand> {
    let maybe_suit_of_flush = hand.suit_of_flush();
    if hand.cards().len() < 5 || maybe_suit_of_flush.is_none() {
        return None;
    }
    let suit = maybe_suit_of_flush.unwrap();
    let cards_of_suits: Vec<&NonJokerCard> = hand.cards().iter().filter(|c| c.suit() == suit).collect();
    assert!(cards_of_suits.len() >= 5);

    let cards = [
        *cards_of_suits[0],
        *cards_of_suits[1],
        *cards_of_suits[2],
        *cards_of_suits[3],
        *cards_of_suits[4],
    ];
    Some(BestFiveHand { cards, rank: HandRank::Flush })
}


#[cfg(test)]
mod test {
    use super::*;
    use playing_card::card:: { CardRank, NonJokerCard, Suit };

    #[test]
    fn returns_none_when_only_4_same_suit_cards_exist() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(5)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_some_when_5_same_suit_cards_exist() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn returns_some_when_6_same_suit_cards_exist() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(1)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn returns_some_when_extra_card_of_another_suit_exists() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Spade, CardRank::new(1)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn result_includes_higher_ranks_of_suit_of_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(6)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(12),
            CardRank::new(10),
            CardRank::new(7),
            CardRank::new(6),
            CardRank::new(5),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn hand_rank_of_result_is_flush() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        assert_eq!(HandRank::Flush, result.unwrap().value().hand_rank)
    }
}