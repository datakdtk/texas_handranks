use super::worker::TrialResult;
use std::collections::HashMap;
use texas_holdem::card::StartingHand;

pub struct AggregationResult {
    example_hand: StartingHand,
    counts: HashMap<TrialResult, usize>
}

impl AggregationResult {
    pub fn new(example_hand: StartingHand) -> Self {
        Self { example_hand, counts: HashMap::new() }
    }

    pub fn csv_head() -> String {
        let cells = [
            "hand",
            "number_of_occurrence",
            "pre_flop_win",
            "flop_win",
            "flop_drop",
            "show_down_win",
            "show_down_lose",
        ];
        cells.join(",").to_string()
    }

    pub fn hand_summary(&self) -> String {
        self.example_hand.summary()
    }

    pub fn count_up(&mut self, result: TrialResult) {
        let c = self.counts.entry(result).or_insert(0);
        *c += 1;
    }

    pub fn total_count(&self) -> usize {
        self.counts.values().fold(0, |sum, count| { sum + count })
    }

    pub fn csv_row(&self) -> String {
        let cells = [
            self.example_hand.summary(),
            self.total_count().to_string(),
            self.counts.get(&TrialResult::PreFlopWin).or(Some(&0)).unwrap().to_string(),
            self.counts.get(&TrialResult::FlopWin).or(Some(&0)).unwrap().to_string(),
            self.counts.get(&TrialResult::FlopDrop).or(Some(&0)).unwrap().to_string(),
            self.counts.get(&TrialResult::ShowDownWin).or(Some(&0)).unwrap().to_string(),
            self.counts.get(&TrialResult::ShowDownLose).or(Some(&0)).unwrap().to_string(),
        ];
        cells.join(",").to_string()
    }
}