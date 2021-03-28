use crate::starting_hand;
use std::sync::mpsc::Sender;
use texas_holdem::Phase;
use texas_holdem::card::{ BestFiveHand, Board, HandRank, StartingHand, TotalHand };

#[derive(Debug, Copy, Clone)]
pub struct TrialResult {
    starting_hand: StartingHand,
    rank: HandRank,
}

impl TrialResult {
    pub fn starting_hand(self) -> StartingHand {
        self.starting_hand
    }

    pub fn hand_rank(self) -> HandRank {
        self.rank
    }
}

pub fn run_worker(sender: Sender<TrialResult>, number_of_players: usize, phase: Phase, trial_count: usize) {
    for _ in 1 ..= trial_count {
        let maybe_result = do_trial(number_of_players, phase);
        if let Some(result) = maybe_result {
            let sending = sender.send(result);
            if let Err(e) = sending {
                println!("channel sending error: {:?}", e);
            }
        }
    }
}

fn do_trial(num_of_players: usize, phase: Phase) -> Option<TrialResult> {
    let mut board =  Board::new();
    let mut hands = board.deal_starting_hands(num_of_players);
    hands.retain(|h| starting_hand::evaluate_hand(*h));
    if hands.len() <= 1 {
        return None;
    }
    board.deal_cards_until(phase);
    Some(find_winner(&hands, &board))
}

fn find_winner(starting_hands: &[StartingHand], board: &Board) -> TrialResult {
    let mut starts_and_bests: Vec<(StartingHand, BestFiveHand)> = starting_hands.iter().map(|starting| {
        let total = TotalHand::new_from_starting_hand_and_board(*starting, board);
        let best = total.find_best_five_hand()
                        .expect("failed to find best five hand. maybe cards are less than 5");
        (*starting, best)
    }).collect();
    starts_and_bests.sort_by(|(_, a_best), (_, b_best)| a_best.value().cmp(&b_best.value()).reverse());
    let (winner_s, winner_b) = starts_and_bests[0];
    TrialResult { starting_hand: winner_s, rank: winner_b.hand_rank()}
}

