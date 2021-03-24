use crate::TotalHand;
use super::{ BestFiveHand, HandRank };
use playing_card::card::NonJokerCard;


pub(super) fn try_to_build_from_total_hand(hand: TotalHand) -> Option<BestFiveHand> {
    let ranks_of_pairs = hand.ranks_of_pairs();
    if hand.cards().len() < 5 || ranks_of_pairs.len() < 2 {
        return None;
    }
    let higher_rank = ranks_of_pairs[0];
    let lower_rank = ranks_of_pairs[1];
    let cards_of_higher_pair: Vec<&NonJokerCard> = hand.cards().iter().filter(|c| c.rank() == higher_rank).collect();
    let cards_of_lower_pair: Vec<&NonJokerCard> = hand.cards().iter().filter(|c| c.rank() == lower_rank).collect();
    let non_pair_card = hand.cards().iter()
                            .filter(|c| c.rank() != higher_rank && c.rank() != lower_rank)
                            .next().expect("kicker of two pairs not found");
    assert_eq!(2, cards_of_higher_pair.len());
    assert_eq!(2, cards_of_lower_pair.len());

    let cards = [
        *cards_of_higher_pair[0],
        *cards_of_higher_pair[1],
        *cards_of_lower_pair[0],
        *cards_of_lower_pair[1],
        *non_pair_card,
    ];
    Some(BestFiveHand { cards, rank: HandRank::TwoPairs })
}


#[cfg(test)]
mod test {
    use super::*;
    use playing_card::card:: { CardRank, NonJokerCard, Suit };

    #[test]
    fn returns_none_when_2_pairs_exist_but_only_4_cards_are_given() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Spade, CardRank::new(10)),
            NonJokerCard::new(Suit::Club, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_none_when_5_cards_are_given_but_no_pair() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_none_when_5_cards_are_given_but_only_1_pair_exists() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_some_when_5_cards_are_given_and_2_pairs_exist() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(2)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn returns_some_when_3_pairs_exist() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(2)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn higher_pair_ranks_come_first_lower_pair_ranks_second_then_kicker_comes_last_in_result() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(10),
            CardRank::new(10),
            CardRank::new(2),
            CardRank::new(2),
            CardRank::new(7),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn third_highest_pair_can_become_kicker_card() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(5)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(7)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(10),
            CardRank::new(10),
            CardRank::new(7),
            CardRank::new(7),
            CardRank::new(5),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn hand_rank_of_result_is_two_pairs() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(hand);
        assert_eq!(HandRank::TwoPairs, result.unwrap().value().hand_rank)
    }
}