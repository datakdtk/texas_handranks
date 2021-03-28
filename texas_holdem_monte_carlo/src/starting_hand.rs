use playing_card::card::CardRank;
use texas_holdem::card::StartingHand;

const TARGET_VALUE: u8 = 3;

pub fn evaluate_hand(hand: StartingHand) -> bool {
    calculate_hand_value(hand) >= TARGET_VALUE
}

fn calculate_hand_value(hand: StartingHand) -> u8 {
    let mut v = 0;
    for c in hand.both_cards().iter() {
        v += evaluate_card_rank(c.rank());
    }
    if hand.is_pair() {
        v += 3;
    }
    if hand.is_suited() {
        v += 1;
    }
    if hand.is_connector() {
        v += 1;
    }
    v
}

fn evaluate_card_rank(rank: CardRank) -> u8 {
    match rank.to_int() {
        1 | 13 => 3,
        10 ..= 12 => 2,
        6 ..= 9 => 1,
        2 ..= 5 => 0,
        _ => panic!(format!("Unexpected card: {}", rank)),
    }
}
