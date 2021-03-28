use std::collections::HashMap;
use texas_holdem::card::{ HandRank, StartingHand };

pub struct AggregationResult {
    example_hand: StartingHand,
    rank_counts: HashMap<HandRank, usize>
}

impl AggregationResult {
    pub fn new(example_hand: StartingHand) -> Self {
        Self { example_hand, rank_counts: HashMap::new() }
    }

    pub fn csv_head() -> String {
        let cells = [
            "hand",
            "actual",
            "adjusted",
            "high_card",
            "pair",
            "two_pairs",
            "three_of_a_king",
            "straight",
            "flush",
            "full_house",
            "four_of_a_kind",
            "straight_flush",
            "royal_flush",
        ];
        cells.join(",").to_string()
    }

    pub fn hand_summary(&self) -> String {
        self.example_hand.summary()
    }

    pub fn count_up(&mut self, rank: HandRank) {
        let c = self.rank_counts.entry(rank).or_insert(0);
        *c += 1;
    }

    pub fn actual_count(&self) -> usize {
        self.rank_counts.values().fold(0, |sum, count| { sum + count })
    }

    pub fn adjusted_count(&self) -> usize {
        let factor = if self.example_hand.is_pair() {
            2
        } else if self.example_hand.is_suited() {
            3
        } else {
            1
        };
        self.actual_count() * factor
    }

    pub fn csv_row(&self) -> String {
        let cells = [
            self.example_hand.summary(),
            self.actual_count().to_string(),
            self.adjusted_count().to_string(),
            self.rank_counts.get(&HandRank::HighCard).or(Some(&0)).unwrap().to_string(),
            self.rank_counts.get(&HandRank::Pair).or(Some(&0)).unwrap().to_string(),
            self.rank_counts.get(&HandRank::TwoPairs).or(Some(&0)).unwrap().to_string(),
            self.rank_counts.get(&HandRank::ThreeOfAKind).or(Some(&0)).unwrap().to_string(),
            self.rank_counts.get(&HandRank::Straight).or(Some(&0)).unwrap().to_string(),
            self.rank_counts.get(&HandRank::Flush).or(Some(&0)).unwrap().to_string(),
            self.rank_counts.get(&HandRank::FullHouse).or(Some(&0)).unwrap().to_string(),
            self.rank_counts.get(&HandRank::FourOfAKind).or(Some(&0)).unwrap().to_string(),
            self.rank_counts.get(&HandRank::StraightFlush).or(Some(&0)).unwrap().to_string(),
            self.rank_counts.get(&HandRank::RoyalFlush).or(Some(&0)).unwrap().to_string(),
        ];
        cells.join(",").to_string()
    }
}