use playing_card::card::CardRank;
use texas_holdem::card::StartingHand;

const TARGET_VALUE: u8 = 5;

pub fn evaluate_hand(hand: StartingHand) -> bool {
    calculate_hand_value(hand) >= TARGET_VALUE
}


fn calculate_hand_value(hand: StartingHand) -> u8 {
    let mut v = 0;
    for c in hand.both_cards().iter() {
        v += evaluate_card_rank(c.rank());
    }
    if hand.is_pair() {
        v += 4;
    }
    if hand.is_suited() {
        v += 1;
    }
    if hand.is_connector() {
        v += 2;
    }
    if hand.is_one_gapper() {
        v += 1;
    }
    v
}

fn evaluate_card_rank(rank: CardRank) -> u8 {
    match rank.to_int() {
        1 | 13 => 4,
        10 ..= 12 => 2,
        6 ..= 9 => 1,
        2 ..= 5 => 0,
        _ => panic!(format!("Unexpected card: {}", rank)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use playing_card::card::{ CardRank, NonJokerCard, Suit };
    
   fn assert_more_than_target(hand: StartingHand) {
         assert!(
            calculate_hand_value(hand) >= TARGET_VALUE,
            format!("hand value should be more than target but actual is {}", evaluate_hand(hand))
        );
        assert!(evaluate_hand(hand));
    }

   fn assert_less_than_target(hand: StartingHand) {
         assert!(
            calculate_hand_value(hand) < TARGET_VALUE,
            format!("hand value should be less than target but actual is {}", evaluate_hand(hand))
        );
        assert!(!evaluate_hand(hand));
    }

    #[test]
    fn hand_of_ace_and_8_should_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Spade, CardRank::Ace),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
        );
        assert_more_than_target(hand);
    }

    #[test]
    fn suited_hand_of_ace_and_3_should_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Spade, CardRank::Ace),
            NonJokerCard::new(Suit::Spade, CardRank::new(3)),
        );
        assert_more_than_target(hand);
    }

    #[test]
    fn non_suited_hand_of_ace_and_2_should_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Spade, CardRank::Ace),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
        );
        assert_more_than_target(hand);
    }

    #[test]
    fn non_suited_hand_of_king_and_2_should_not_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Spade, CardRank::King),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
        );
        assert_less_than_target(hand);
    }

    #[test]
    fn suited_hand_of_king_and_2_should_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Spade, CardRank::King),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
        );
        assert_more_than_target(hand);
    }

    #[test]
    fn suited_hand_of_queen_and_2_should_not_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Spade, CardRank::Queen),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
        );
        assert_less_than_target(hand);
    }

    #[test]
    fn non_suited_hand_of_queen_and_10_should_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Spade, CardRank::Queen),
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
        );
        assert_more_than_target(hand);
    }

    #[test]
    fn non_suited_hand_of_queen_and_8_should_not_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Spade, CardRank::Queen),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
        );
        assert_less_than_target(hand);
    }

    #[test]
    fn non_suited_hand_of_jack_and_10_should_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Spade, CardRank::Jack),
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
        );
        assert_more_than_target(hand);
    }

    #[test]
    fn non_suited_hand_of_jack_and_9_should_not_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Spade, CardRank::Jack),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
        );
        assert_less_than_target(hand);
    }

    #[test]
    fn suited_hand_of_10_and_9_should_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Spade, CardRank::new(9)),
        );
        assert_more_than_target(hand);
    }

    #[test]
    fn non_suited_hand_of_9_and_8_should_not_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Spade, CardRank::new(9)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
        );
        assert_less_than_target(hand);
    }

    #[test]
    fn suited_hand_of_6_and_5_should_not_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Heart, CardRank::new(6)),
            NonJokerCard::new(Suit::Spade, CardRank::new(5)),
        );
        assert_less_than_target(hand);
    }

    #[test]
    fn pair_of_7_should_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Spade, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
        );
        assert_more_than_target(hand);
    }

    #[test]
    fn pair_of_4_should_not_satisfy_target() {
        let hand = StartingHand::new( 
            NonJokerCard::new(Suit::Spade, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
        );
        assert_less_than_target(hand);
    }
}