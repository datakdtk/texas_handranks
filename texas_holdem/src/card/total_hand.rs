use super::{ Board, StartingHand };
use playing_card::card:: { CardRank, NonJokerCard, Suit };
use std::collections::HashMap;

/// Set of all cards being available for making hand ranks
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TotalHand {
    cards: Vec<NonJokerCard>,
    rank_counts: HashMap<CardRank, u8>,
    suit_counts: HashMap<Suit, u8>,
    heads_of_straight: Vec<CardRank>, 
}

impl TotalHand {
    pub fn new(cards: &[NonJokerCard]) -> Self {
        assert!(cards.len() <= 7, "The maximum number of cards in TotalHand is 7");
        let mut sortable_cards: Vec<NonJokerCard> = cards.iter().map(|c| *c).collect();
        sortable_cards.sort_by(|a, b| super::cmp_cards(*a, *b).reverse());
        let mut rank_counts = HashMap::new();
        let mut suit_counts = HashMap::new();
        let mut straight_counter: StraightCounter = Default::default();

        for c in sortable_cards.iter() {
            let rc = rank_counts.entry(c.rank()).or_insert(0);
            *rc += 1;
            let sc = suit_counts.entry(c.suit()).or_insert(0);
            *sc += 1;
            straight_counter.try_to_count_up(c.rank());
        }

        Self {
            cards: sortable_cards,
            rank_counts,
            suit_counts,
            heads_of_straight: straight_counter.determined_heads_of_straight(),
        }
    }

    pub fn new_from_starting_hand_and_board(hand: StartingHand, board: &Board) -> Self {
        let cards = [
            &hand.both_cards()[..],
            &board.cards()[..],
        ];
        Self::new(&cards.concat())
    }

    /// Returns all cards sorted in descending order.
    pub fn cards(&self) -> &[NonJokerCard] {
        &self.cards
    }

    /// Returns ranks of exactly 2 cards.
    /// Higher rank comes first.
    pub fn ranks_of_pairs(&self) -> Vec<CardRank> {
        let mut v: Vec<CardRank> = self.rank_counts.iter()
                                       .filter(|(_k, v)| **v == 2)
                                       .map(|(k, _v)| *k)
                                       .collect();
        v.sort_by(|a, b| super::cmp_card_ranks(*a, *b).reverse());
        v
    }

    /// Returns ranks of exactly 3 cards.
    /// Higher rank comes first.
    pub fn ranks_of_sets(&self) -> Vec<CardRank> {
        let mut v: Vec<CardRank> = self.rank_counts.iter()
                                       .filter(|(_k, v)| **v == 3)
                                       .map(|(k, _v)| *k)
                                       .collect();
        v.sort_by(|a, b| super::cmp_card_ranks(*a, *b).reverse());
        v
    }

    /// Returns a rank of exactly 4 cards if exists.
    pub fn rank_of_quads(&self) -> Option<CardRank> {
        self.rank_counts.iter()
            .filter(|(_k, v)| **v == 4)
            .map(|(k, _v)| *k)
            .last()
    }

    /// Returns a suit of 5 or more cards if exists.
    pub fn suit_of_flush(&self) -> Option<Suit> {
        self.suit_counts.iter()
            .filter(|(_k, v)| **v >= 5)
            .map(|(k, _v)| *k)
            .last()
    }

    pub fn head_ranks_of_straight(&self) -> Vec<CardRank> {
        self.heads_of_straight.clone()
    }
}

/// Helper struct to check sequence of Straight in building TotalHand struct.
#[derive(Debug, Default)]
struct StraightCounter {
    current_head: Option<CardRank>,
    current_count: u8,
    previous_rank: Option<CardRank>,
    determined_heads: Vec<CardRank>,
    has_ace: bool,
}

impl StraightCounter {
    /// new_rank is expected to be given in descending order
    fn try_to_count_up(&mut self, new_rank: CardRank) {
        if new_rank == CardRank::Ace {
            self.has_ace = true; // memorize ace to create 5 to ace straight later.
        }
        // Initialize at the first time of method call
        if self.current_head == None {
            self.initialize_counting(new_rank);
        }
        // Do not count same rank twice
        if self.previous_rank == Some(new_rank) {
            return;
        }

        let connecting_rank = match self.previous_rank {
            Some(CardRank::Ace) if self.current_head == Some(CardRank::Ace) => CardRank::King,
            Some(x) if x != CardRank::Ace => CardRank::new(x.to_int() - 1),
            _ => return, // Sequence can not get longer
        };

        // Update previous rank after checking connecting_rank
        self.previous_rank = Some(new_rank);

        // Sequence has broken. Restart counting from 1 with new head
        if new_rank != connecting_rank {
            self.initialize_counting(new_rank);
            return;
        }

        // Count up if new_rank is connectable
        self.current_count += 1;

        // A straight sequence is completed. Save current_head and update current_head and current_count.
        if self.current_count >= 5 {
            assert_eq!(5, self.current_count);
            self.determined_heads.push(self.current_head.expect("current_head_is_missing"));
            self.current_head = match self.current_head {
                Some(CardRank::Ace) => Some(CardRank::King),
                Some(x) if x.to_int() > 2 => Some(CardRank::new(x.to_int() - 1)),
                x => panic!(format!("current_head is unexpected value when completing straight: {:?}", x)),
            };
            self.current_count = 4;
        }

        // Complete 5 to ace straight if current sequence is from 5 to 2
        let head_five = CardRank::new(5);
        if self.current_head == Some(head_five) && self.current_count == 4 && self.has_ace {
            self.determined_heads.push(head_five);
            self.current_count = 0; // never expected to be counted up later.
        }
    }

    fn initialize_counting(&mut self, first_rank: CardRank) {
        self.current_head = Some(first_rank);
        self.current_count = 1;
        self.previous_rank = Some(first_rank);
    }

    fn determined_heads_of_straight(self) -> Vec<CardRank> {
        self.determined_heads
    }
}



#[cfg(test)]
mod test {
    use crate::Phase;
    use super::*;

    #[test]
    fn can_construct_from_hand_and_board() {
        let mut board = Board::new();
        let starting_hand = board.deal_starting_hands(1)[0];
        board.deal_cards_until(Phase::River);
        let total_hand = TotalHand::new_from_starting_hand_and_board(starting_hand, &board);
        assert_eq!(7, total_hand.cards().len());
    }

    #[test]
    fn can_create_with_no_card() {
        TotalHand::new(&[]); // Don' be panic!
    }

    #[test]
    #[should_panic(expected = "number of cards")]
    fn should_panic_when_8_cards_are_given() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(6)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
        ];
        TotalHand::new(&cards); // Be panic!
    }

    #[test]
    fn cards_should_get_sorted_in_descending_order() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&cards);
        let sorted = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
        ];
        assert_eq!(sorted, hand.cards());
    }

    #[test]
    fn can_detect_one_pair() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&cards);
        let expected = vec![
            CardRank::new(13),
        ];
        assert_eq!(expected, hand.ranks_of_pairs());
    }

    #[test]
    fn can_detect_two_pairs() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(9)),
        ];
        let hand = TotalHand::new(&cards);
        let expected = vec![
            CardRank::new(13),
            CardRank::new(9),
        ];
        assert_eq!(expected, hand.ranks_of_pairs());
    }

    #[test]
    fn set_is_not_pair() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(13)),
        ];
        let hand = TotalHand::new(&cards);
        let expected:Vec<CardRank> = vec![
        ];
        assert_eq!(expected, hand.ranks_of_pairs());
    }

    #[test]
    fn can_detect_one_set() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(13)),
        ];
        let hand = TotalHand::new(&cards);
        let expected = vec![
            CardRank::new(13),
        ];
        assert_eq!(expected, hand.ranks_of_sets());
    }

    #[test]
    fn can_detect_two_sets() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(4)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(13)),
            NonJokerCard::new(Suit::Club, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
        ];
        let hand = TotalHand::new(&cards);
        let expected = vec![
            CardRank::new(13),
            CardRank::new(4),
        ];
        assert_eq!(expected, hand.ranks_of_sets());
    }

    #[test]
    fn quads_is_not_sets() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(13)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(13)),
            NonJokerCard::new(Suit::Spade, CardRank::new(4)),
        ];
        let hand = TotalHand::new(&cards);
        let expected:Vec<CardRank> = vec![
        ];
        assert_eq!(expected, hand.ranks_of_sets());
    }

    #[test]
    fn can_detect_quads() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Spade, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(13)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(13)),
            NonJokerCard::new(Suit::Spade, CardRank::new(4)),
        ];
        let hand = TotalHand::new(&cards);
        assert_eq!(Some(CardRank::new(13)), hand.rank_of_quads());
    }

    #[test]
    fn none_is_returned_when_no_quads() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(13)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(13)),
            NonJokerCard::new(Suit::Spade, CardRank::new(4)),
        ];
        let hand = TotalHand::new(&cards);
        assert_eq!(None, hand.rank_of_quads());
    }

    #[test]
    fn can_detect_flush_with_5_same_suits() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(13)),
            NonJokerCard::new(Suit::Spade, CardRank::new(4)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
        ];
        let hand = TotalHand::new(&cards);
        assert_eq!(Some(Suit::Heart), hand.suit_of_flush());
    }

    #[test]
    fn can_detect_flush_with_6_same_suits() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(1)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
        ];
        let hand = TotalHand::new(&cards);
        assert_eq!(Some(Suit::Heart), hand.suit_of_flush());
    }

    #[test]
    fn none_is_returned_when_no_flush() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(13)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(13)),
            NonJokerCard::new(Suit::Spade, CardRank::new(4)),
        ];
        let hand = TotalHand::new(&cards);
        assert_eq!(None, hand.suit_of_flush());
    }

    #[test]
    fn four_cards_are_not_straight() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(3)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(5)),
            NonJokerCard::new(Suit::Spade, CardRank::new(4)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_of_exactly_five_cards() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(3)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(5)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(6)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
            CardRank::new(6),
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_with_extra_card_higher_than_top() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(8)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(5)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(6)),
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
            CardRank::new(8),
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_with_extra_card_lower_than_tail() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(8)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(5)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(6)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
            CardRank::new(8),
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_with_higher_and_lower_extra_cards() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(8)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(5)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(6)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
            CardRank::new(8),
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_with_duplicate_card_ranks_on_head() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(8)),
            NonJokerCard::new(Suit::Spade, CardRank::new(8)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(5)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(6)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
            CardRank::new(8),
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_with_duplicate_card_ranks_on_tail() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(8)),
            NonJokerCard::new(Suit::Spade, CardRank::new(4)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(5)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(6)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
            CardRank::new(8),
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_with_duplicate_card_ranks_in_middle() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(8)),
            NonJokerCard::new(Suit::Spade, CardRank::new(5)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(5)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(6)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
            CardRank::new(8),
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_with_two_pairs_of_duplicate_card_ranks() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Spade, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(8)),
            NonJokerCard::new(Suit::Spade, CardRank::new(5)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(5)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(6)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
            CardRank::new(8),
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_with_3_duplicate_cards_ranks() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
            NonJokerCard::new(Suit::Club, CardRank::new(8)),
            NonJokerCard::new(Suit::Spade, CardRank::new(5)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(5)),
            NonJokerCard::new(Suit::Club, CardRank::new(5)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(6)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
            CardRank::new(8),
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_from_ace_to_10() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(12)),
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Club, CardRank::new(11)),
            NonJokerCard::new(Suit::Spade, CardRank::new(10)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(1)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
            CardRank::new(1),
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_from_5_to_ace() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Club, CardRank::new(4)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(1)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
            CardRank::new(5),
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn not_straight_from_5_to_ace_when_2_dropped() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(6)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Club, CardRank::new(4)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(1)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn not_straight_from_5_to_ace_when_5_dropped() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(6)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Club, CardRank::new(4)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(1)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_cannot_get_over_ace() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(13)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Club, CardRank::new(12)),
            NonJokerCard::new(Suit::Spade, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(1)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_with_6_cards_sequence() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(6)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Club, CardRank::new(4)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(7)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
            CardRank::new(7),
            CardRank::new(6),
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn straight_with_7_cards_sequence() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(8)),
            NonJokerCard::new(Suit::Heart, CardRank::new(6)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Club, CardRank::new(4)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(2)),
            NonJokerCard::new(Suit::Diamond, CardRank::new(7)),
        ];
        let hand = TotalHand::new(&cards);
        let expected: Vec<CardRank> = vec![
            CardRank::new(8),
            CardRank::new(7),
            CardRank::new(6),
        ];
        assert_eq!(expected, hand.head_ranks_of_straight());
    }
}