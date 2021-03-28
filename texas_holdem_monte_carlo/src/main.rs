extern crate playing_card;
extern crate texas_holdem;

pub mod aggregator;
pub mod starting_hand;
pub mod worker;

use aggregator::{ AggregationResult };
use texas_holdem::Phase;
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;

fn main() {
    let worker_count = 10;
    let trial_per_worker = 100000;
    let num_of_players = 6;
    let phase = Phase::Flop;
    
    let (tx, rx) = mpsc::channel();
    for _ in 1 ..= worker_count {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            worker::run_worker(tx_clone, num_of_players, phase, trial_per_worker);
        });
    }
    drop(tx);

    let mut aggregations = HashMap::new();
    for received in rx {
        let starting_hand = received.starting_hand();
        let count = aggregations.entry(starting_hand.summary())
                                .or_insert(AggregationResult::new(starting_hand));
        count.count_up(received.hand_rank());
    }

    let mut results: Vec<&AggregationResult> = aggregations.values().collect();
    results.sort_by(|a, b| a.adjusted_count().cmp(&b.adjusted_count()).reverse());

    println!("{}", AggregationResult::csv_head());
    for r in results {
        println!("{}", r.csv_row());
    }
}
