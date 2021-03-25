use crate::TotalHand;
use super::{ BestFiveHand, HandRank };
use playing_card::card::NonJokerCard;


pub(super) fn try_to_build_from_total_hand(hand: &TotalHand) -> Option<BestFiveHand> {
    let rank_of_pairs = hand.ranks_of_pairs();
    if hand.cards().len() < 5 || rank_of_pairs.is_empty() {
        return None;
    }
    let rank= rank_of_pairs[0];
    let cards_of_pair: Vec<&NonJokerCard> = hand.cards().iter().filter(|c| c.rank() == rank).collect();
    let non_pair_cards: Vec<&NonJokerCard> =  hand.cards().iter().filter(|c| c.rank() != rank).collect();
    assert_eq!(2, cards_of_pair.len());
    assert!(non_pair_cards.len() >= 3);

    let cards = [
        *cards_of_pair[0],
        *cards_of_pair[1],
        *non_pair_cards[0],
        *non_pair_cards[1],
        *non_pair_cards[2],
    ];
    Some(BestFiveHand { cards, rank: HandRank::Pair })
}


#[cfg(test)]
mod test {
    use super::*;
    use playing_card::card:: { CardRank, NonJokerCard, Suit };

    #[test]
    fn returns_none_when_is_paired_but_only_4_cards_are_given() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Spade, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
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
        let result = try_to_build_from_total_hand(&hand);
        if let Some(_) = result {
            panic!("Result is expected to be None");
        }
    }

    #[test]
    fn returns_some_when_5_cards_are_given_and_is_paired() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        if let None = result {
            panic!("Result is expected not to be None");
        }
    }

    #[test]
    fn rank_of_pair_comes_first_in_result_value_and_other_ranks_are_sorted() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        let result_ranks = result.unwrap().value().card_ranks;
        let expected_ranks = [
            CardRank::new(2),
            CardRank::new(2),
            CardRank::new(10),
            CardRank::new(9),
            CardRank::new(7),
        ];
        assert_eq!(expected_ranks, result_ranks)
    }

    #[test]
    fn hand_rank_of_result_is_pair() {
        let given_cards = [
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&given_cards);
        let result = try_to_build_from_total_hand(&hand);
        assert_eq!(HandRank::Pair, result.unwrap().value().hand_rank)
    }
}