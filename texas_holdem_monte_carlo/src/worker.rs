use crate::starting_hand;
use crate::flop_hand;
use std::sync::mpsc::Sender;
use texas_holdem::Phase;
use texas_holdem::card::{ BestFiveHand, Board, StartingHand, TotalHand };


#[derive(Debug, Copy, Clone)]
pub struct TrialResultOfHand {
    starting_hand: StartingHand,
    result: TrialResult,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum TrialResult {
    PreFlopWin,
    FlopDrop,
    FlopWin,
    ShowDownWin,
    ShowDownLose,
}


impl TrialResultOfHand {
    pub fn starting_hand(self) -> StartingHand {
        self.starting_hand
    }

    pub fn result(self) -> TrialResult {
        self.result
    }
}

pub fn run_worker(sender: Sender<TrialResultOfHand>, number_of_players: usize, trial_count: usize) {
    for _ in 1 ..= trial_count {
        do_trial(&sender, number_of_players);
    }
}

fn do_trial(sender: &Sender<TrialResultOfHand>, num_of_players: usize) {
    let mut board =  Board::new();
    let mut hands = board.deal_starting_hands(num_of_players);
    hands.retain(|h| starting_hand::evaluate_hand(*h));
    if hands.len() <= 1 {
        let winner = hands.get(0);
        if let Some(&h) = winner {
            let _ = sender.send(TrialResultOfHand { starting_hand: h, result: TrialResult::PreFlopWin });
        }
    }
    board.deal_cards_until(Phase::Flop);
    let hands = flop_check(sender, hands, &board);
    if hands.len() <= 1 {
        let winner = hands.get(0);
        if let Some(&h) = winner {
            let _ = sender.send(TrialResultOfHand { starting_hand: h, result: TrialResult::FlopWin });
        }
        return;
    }
    board.deal_cards_until(Phase::Flop);
    show_down_check(&sender, hands, &board);
}

fn flop_check(sender: &Sender<TrialResultOfHand>, starting_hands: Vec<StartingHand>, board: &Board) -> Vec<StartingHand> {
    let starts_and_totals: Vec<(StartingHand, TotalHand)> = starting_hands.iter().map(|starting| {
        let total = TotalHand::new_from_starting_hand_and_board(*starting, board);
        (*starting, total)
    }).collect();
    let mut winners = Vec::new();
    let mut losers = Vec::new();
    for (s, t) in starts_and_totals {
        if flop_hand::evaluate_hand(t) {
            winners.push(s);
        } else {
            losers.push(s);
        }
    };
    if winners.is_empty() {
        starting_hands
    } else {
        for h in losers {
            let _ = sender.send(TrialResultOfHand { starting_hand: h, result: TrialResult::FlopDrop });
        }
        winners
    }
}

fn show_down_check(sender: &Sender<TrialResultOfHand>, starting_hands: Vec<StartingHand>, board: &Board) {
    let mut starts_and_bests: Vec<(StartingHand, BestFiveHand)> = starting_hands.iter().map(|starting| {
        let total = TotalHand::new_from_starting_hand_and_board(*starting, board);
        let best = total.find_best_five_hand().unwrap();
        (*starting, best)
    }).collect();
    starts_and_bests.sort_by(|(_, a_best), (_, b_best)| a_best.value().cmp(&b_best.value()).reverse());

    let winner = starts_and_bests[0];
    let _ = sender.send(TrialResultOfHand { starting_hand: winner.0, result: TrialResult::ShowDownWin });

    for loser in starts_and_bests[1..].iter() {
        let _ = sender.send(TrialResultOfHand { starting_hand: loser.0, result: TrialResult::ShowDownLose });
    }
}

