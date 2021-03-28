use texas_holdem::card::{ TotalHand, HandRank };

pub fn evaluate_hand(hand: TotalHand) -> bool {
    let best = hand.find_best_five_hand().unwrap();
    match best.hand_rank() {
        HandRank::HighCard => false,
        _ => true,
    }
}