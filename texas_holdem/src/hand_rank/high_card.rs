use super::{ BestFiveHand, HandRank };
use crate::TotalHand;

pub(super) fn try_to_build_from_total_hand(hand: TotalHand) -> Option<BestFiveHand> {
    if hand.cards().len() < 5 {
        return None;
    }

    let cards = [
        hand.cards()[0],
        hand.cards()[1],
        hand.cards()[2],
        hand.cards()[3],
        hand.cards()[4],
    ];
    Some(BestFiveHand { cards, rank: HandRank::HighCard })
}


#[cfg(test)]
mod test {
    use super::*;
    use playing_card::card:: { CardRank, NonJokerCard, Suit };

    #[test]
    fn returns_none_when_only_4_cards_are_given() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_some_when_5_cards_are_given() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn card_ranks_of_result_is_sorted_by_rank() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(12)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(12),
            CardRank::new(10),
            CardRank::new(9),
            CardRank::new(7),
            CardRank::new(4),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn hand_rank_of_result_is_high_card() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(12)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        assert_eq!(HandRank::HighCard, result.unwrap().value().hand_rank)
    }
}
