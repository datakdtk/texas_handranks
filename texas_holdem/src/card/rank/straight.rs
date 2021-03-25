use crate::card::{ BestFiveHand, HandRank, TotalHand};
use playing_card::card:: { CardRank, NonJokerCard };


pub(super) fn try_to_build_from_total_hand(hand: &TotalHand) -> Option<BestFiveHand> {
    let head_ranks_of_straight = hand.head_ranks_of_straight();
    if head_ranks_of_straight.is_empty() {
        return None;
    }
    let head = head_ranks_of_straight[0];
    let head_rank_int = if head == CardRank::Ace { 14 } else { head.to_int() };
    let ranks = [
        head,
        CardRank::new(head_rank_int - 1),
        CardRank::new(head_rank_int - 2),
        CardRank::new(head_rank_int - 3),
        CardRank::new(head_rank_int - 4),
    ];

    let card_vec: Vec<NonJokerCard> = ranks.iter().map(|r| {
        *hand.cards().iter().filter(|c| c.rank() == *r).next()
             .expect(&format!("a card of rank {} is not found for straight", r.to_int()))
    }).collect();

    assert_eq!(5, card_vec.len());

    let cards = [
        card_vec[0],
        card_vec[1],
        card_vec[2],
        card_vec[3],
        card_vec[4],
    ];
    Some(BestFiveHand { cards, rank: HandRank::Straight })
}


#[cfg(test)]
mod test {
    use super::*;
    use playing_card::card:: { CardRank, NonJokerCard, Suit };

    #[test]
    fn returns_none_when_not_straight() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
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
    fn returns_some_when_straight() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn can_build_ace_high_straight() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(13)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn can_build_five_high_straight() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(5)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn result_contains_ranks_of_straight() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(9)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(1)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
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
    fn result_contains_higher_straight_when_sequence_is_longer_than_5() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(9)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(13)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
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
    fn hand_rank_of_result_is_straight() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(11)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        assert_eq!(HandRank::Straight, result.unwrap().value().hand_rank)
    }
}