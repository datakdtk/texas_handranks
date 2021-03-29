use playing_card::card::NonJokerCard;
use std::cmp::Ordering;
use texas_holdem::card::cmp_cards;
use texas_holdem::card::cmp_card_ranks;
use texas_holdem::card::{ HandRank, StartingHand, TotalHand, };

pub fn evaluate_hand(hand: StartingHand, flop: [NonJokerCard; 3]) -> bool {
    let greatest_flop_rank = flop.iter().max_by(|a, b| cmp_cards(**a, **b)).unwrap().rank();
    // ok if starting hand is 2 overs
    if cmp_card_ranks(hand.lower_card().rank(), greatest_flop_rank) >= Ordering::Greater {
        return true;
    }

    let total_hand = TotalHand::new(&[
        hand.higher_card(),
        hand.lower_card(),
        flop[0],
        flop[1],
        flop[2],
    ]);
    
    if total_hand.is_open_end_straight_draw() || total_hand.is_flush_draw() {
        return true;
    }
    
    let best = total_hand.find_best_five_hand().unwrap();
    match best.hand_rank() {
        HandRank::HighCard => false,
        _ => true,
    }
}