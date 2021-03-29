extern crate playing_card;
extern crate texas_holdem;

pub mod aggregator;
pub mod flop_hand;
pub mod starting_hand;
pub mod worker;

use aggregator::{ AggregationResult };
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;

fn main() {
    let worker_count = 8;
    let trial_per_worker = 500_000;
    let num_of_players = 6;
    
    let (tx, rx) = mpsc::channel();
    for _ in 1 ..= worker_count {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            worker::run_worker(tx_clone, num_of_players, trial_per_worker);
        });
    }
    drop(tx);

    let mut aggregations = HashMap::new();
    for received in rx {
        let starting_hand = received.starting_hand();
        let count = aggregations.entry(starting_hand.summary())
                                .or_insert(AggregationResult::new(starting_hand));
        count.count_up(received.result());
    }

    let results: Vec<&AggregationResult> = aggregations.values().collect();

    println!("{}", AggregationResult::csv_head());
    for r in results {
        println!("{}", r.csv_row());
    }
}
