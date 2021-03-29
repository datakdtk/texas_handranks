use super::{ Board, StartingHand };
use playing_card::card:: { CardRank, NonJokerCard, Suit };
use std::collections::{ HashMap, HashSet };

/// Set of all cards being available for making hand ranks
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TotalHand {
    cards: Vec<NonJokerCard>,
    rank_counts: HashMap<CardRank, u8>,
    suit_counts: HashMap<Suit, u8>,
    ranks_of_straight: RanksOfStraight,
}

impl TotalHand {
    pub fn new(cards: &[NonJokerCard]) -> Self {
        assert!(cards.len() <= 7, "The maximum number of cards in TotalHand is 7");
        let mut sortable_cards: Vec<NonJokerCard> = cards.iter().map(|c| *c).collect();
        sortable_cards.sort_by(|a, b| super::cmp_cards(*a, *b).reverse());
        let mut rank_counts = HashMap::new();
        let mut suit_counts = HashMap::new();
        // 13 bit integer that indicates existence of cards of each number
        let mut bit_of_cards = 0u16;
        for c in sortable_cards.iter() {
            let rc = rank_counts.entry(c.rank()).or_insert(0);
            *rc += 1;
            let sc = suit_counts.entry(c.suit()).or_insert(0);
            *sc += 1;
            bit_of_cards = bit_of_cards | 2u16.pow(u32::from(c.rank().to_int()-1));
        }

        let ranks_of_straight = RanksOfStraight::calculate(bit_of_cards);

        Self {
            cards: sortable_cards,
            rank_counts,
            suit_counts,
            ranks_of_straight,
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

    /// Returns true if at least one suit has 5 or more cards.
    pub fn is_flush_draw(&self) -> bool {
        self.suit_counts.iter()
            .any(|(_k, v)| *v >= 4)
    }

    pub fn head_ranks_of_straight(&self) -> &[CardRank] {
        &self.ranks_of_straight.ranks_of_head
    }

    pub fn straight_draw_ranks(&self) -> &[CardRank] {
        &self.ranks_of_straight.ranks_of_draw
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RanksOfStraight {
    ranks_of_head: Vec<CardRank>,
    ranks_of_draw: Vec<CardRank>,
}

impl RanksOfStraight {
    fn calculate(bit_of_cards: u16) -> Self {
        let mut ranks_of_head = Vec::new();
        let mut rank_set_of_draw = HashSet::new();

        // pair of bit representing straight and its head rank
        let bits_of_straight = [
            (0b1111000000001u16, CardRank::Ace),
            (0b1111100000000u16, CardRank::King),
            (0b0111110000000u16, CardRank::Queen),
            (0b0011111000000u16, CardRank::Jack),
            (0b0001111100000u16, CardRank::new(10)),
            (0b0000111110000u16, CardRank::new(9)),
            (0b0000011111000u16, CardRank::new(8)),
            (0b0000001111100u16, CardRank::new(7)),
            (0b0000000111110u16, CardRank::new(6)),
            (0b0000000011111u16, CardRank::new(5)),
        ];

        let card_bit_map: HashMap<u16, CardRank> = (1 ..= 13)
            .map(|n| (2u16.pow(u32::from(n-1)), CardRank::new(n)))
            .collect();

        for (target_bit, head_rank) in &bits_of_straight {
            let and = *target_bit & bit_of_cards;
            // when straight is completed
            if and == *target_bit { 
                ranks_of_head.push(*head_rank);
                continue;
            }

            let lacked = target_bit ^ and;
            // when the lacked bit exactly equals to a power of 2
            // it means there is only one card lacking (= straight_draw)
            if lacked > 0 && (lacked & (lacked - 1) == 0) {
                rank_set_of_draw.insert(card_bit_map[&lacked]);
            }
        };

        let mut ranks_of_draw: Vec<CardRank> = rank_set_of_draw.iter().map(|r| *r).collect();
        ranks_of_draw.sort_by(|a, b| super::cmp_card_ranks(*a, *b).reverse());
        Self { ranks_of_head, ranks_of_draw}
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
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
        assert_eq!(&expected, hand.head_ranks_of_straight());
    }

    #[test]
    fn same_suit_4_cards_are_flush_draw() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(6)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Heart, CardRank::new(4)),
        ];
        let hand = TotalHand::new(&cards);
        assert!(hand.is_flush_draw());
    }

    #[test]
    fn same_suit_3_cards_are_not_flush_draw() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(6)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(2)),
            NonJokerCard::new(Suit::Club, CardRank::new(4)),
        ];
        let hand = TotalHand::new(&cards);
        assert!(!hand.is_flush_draw());
    }

    #[test]
    fn open_end_straight_draw_has_2_draw_ranks() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(6)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Club, CardRank::new(4)),
        ];
        let hand = TotalHand::new(&cards);
        let expected = [
            CardRank::new(7),
            CardRank::new(2),
        ];
        assert_eq!(&expected, hand.straight_draw_ranks());
    }

    #[test]
    fn inside_straight_draw_has_1_draw_rank() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(5)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Club, CardRank::new(4)),
        ];
        let hand = TotalHand::new(&cards);
        let expected = [
            CardRank::new(6),
        ];
        assert_eq!(&expected, hand.straight_draw_ranks());
    }

    #[test]
    fn not_open_end_straight_draw_but_has_2_draw_rank() {
        let cards = vec![
            NonJokerCard::new(Suit::Heart, CardRank::new(10)),
            NonJokerCard::new(Suit::Heart, CardRank::new(9)),
            NonJokerCard::new(Suit::Heart, CardRank::new(7)),
            NonJokerCard::new(Suit::Heart, CardRank::new(6)),
            NonJokerCard::new(Suit::Heart, CardRank::new(3)),
            NonJokerCard::new(Suit::Club, CardRank::new(4)),
        ];
        let hand = TotalHand::new(&cards);
        let expected = [
            CardRank::new(8),
            CardRank::new(5),
        ];
        assert_eq!(&expected, hand.straight_draw_ranks());
    }
}