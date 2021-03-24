use crate::TotalHand;
use super::{ BestFiveHand, HandRank };
use playing_card::card::NonJokerCard;


pub(super) fn try_to_build_from_total_hand(hand: TotalHand) -> Option<BestFiveHand> {
    if hand.cards().len() < 5 || hand.rank_of_quads().is_none() {
        return None;
    }
    let rank_of_quads = hand.rank_of_quads().unwrap();
    let cards_of_quads: Vec<&NonJokerCard> = hand.cards().iter().filter(|c| c.rank() == rank_of_quads).collect();
    let non_quad_cards: Vec<&NonJokerCard> =  hand.cards().iter().filter(|c| c.rank() != rank_of_quads).collect();
    assert_eq!(4, cards_of_quads.len());
    assert!(non_quad_cards.len() >= 1);

    let cards = [
        *cards_of_quads[0],
        *cards_of_quads[1],
        *cards_of_quads[2],
        *cards_of_quads[3],
        *non_quad_cards[0],
    ];
    Some(BestFiveHand { cards, rank: HandRank::FourOfAKind })
}


#[cfg(test)]
mod test {
    use super::*;
    use playing_card::card:: { CardRank, NonJokerCard, Suit };

    #[test]
    fn returns_none_when_quads_exist_but_only_4_cards_are_given() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(10)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(10)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_none_when_5_cards_are_given_but_no_quads() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(10)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_some_when_5_cards_are_given_and_quads_exit() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(10)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(10)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn rank_of_quads_comes_first_in_result_value_and_other_ranks_are_sorted() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Club, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(2),
            CardRank::new(2),
            CardRank::new(2),
            CardRank::new(2),
            CardRank::new(10),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn hand_rank_of_result_is_four_of_a_kind() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        assert_eq!(HandRank::FourOfAKind, result.unwrap().value().hand_rank)
    }
}