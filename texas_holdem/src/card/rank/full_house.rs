use crate::card::{ BestFiveHand, HandRank, TotalHand};
use playing_card::card::NonJokerCard;


pub(super) fn try_to_build_from_total_hand(hand: &TotalHand) -> Option<BestFiveHand> {
    let ranks_of_sets = hand.ranks_of_sets();
    let ranks_of_pairs = hand.ranks_of_pairs();
    if hand.cards().len() < 5 || ranks_of_sets.is_empty() {
        return None;
    }
    if ranks_of_sets.len() < 2 && ranks_of_pairs.is_empty() {
        return None;
    }


    let rank_of_set = ranks_of_sets[0];
    let rank_of_pair = *ranks_of_sets.get(1).or(ranks_of_pairs.get(0)).unwrap();
    let cards_of_set: Vec<&NonJokerCard> = hand.cards().iter().filter(|c| c.rank() == rank_of_set).collect();
    let cards_of_pair: Vec<&NonJokerCard> = hand.cards().iter().filter(|c| c.rank() == rank_of_pair).collect();
    assert_eq!(3, cards_of_set.len());
    assert!(cards_of_pair.len() >= 2);

    let cards = [
        *cards_of_set[0],
        *cards_of_set[1],
        *cards_of_set[2],
        *cards_of_pair[0],
        *cards_of_pair[1],
    ];
    Some(BestFiveHand { cards, rank: HandRank::FullHouse })
}


#[cfg(test)]
mod test {
    use super::*;
    use playing_card::card:: { CardRank, NonJokerCard, Suit };

    #[test]
    fn returns_none_when_only_set_exists() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(10)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_none_when_only_pairs_exists() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_some_when_both_set_and_pair_exist() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(2)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
            NonJokerCard::new(Suit::Spade, CardRank::new(1)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn returns_some_when_2_sets_exist() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(2)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn rank_of_set_come_first_and_rank_of_pair_comes_second_in_result() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(2),
            CardRank::new(2),
            CardRank::new(2),
            CardRank::new(10),
            CardRank::new(10),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn lower_set_becomes_pair_when_2_sets_exist() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(10)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(10),
            CardRank::new(10),
            CardRank::new(10),
            CardRank::new(2),
            CardRank::new(2),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn higher_pair_appears_in_result_when_2_pairs_exist() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(10)),
            NonJokerCard::new(Suit::Club, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(2),
            CardRank::new(2),
            CardRank::new(2),
            CardRank::new(10),
            CardRank::new(10),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn hand_rank_of_result_is_full_house() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(2)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        assert_eq!(HandRank::FullHouse, result.unwrap().value().hand_rank)
    }
}