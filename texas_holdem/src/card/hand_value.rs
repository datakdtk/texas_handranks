use super::HandRank;
use playing_card::card::CardRank;
use std::cmp;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct HandValue {
    pub(super) hand_rank: HandRank,
    pub(super) card_ranks: [CardRank; 5],
}

impl cmp::Ord for HandValue {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.hand_rank.cmp(&other.hand_rank)
            .then(super::cmp_card_ranks(self.card_ranks[0], other.card_ranks[0]))
            .then(super::cmp_card_ranks(self.card_ranks[1], other.card_ranks[1]))
            .then(super::cmp_card_ranks(self.card_ranks[2], other.card_ranks[2]))
            .then(super::cmp_card_ranks(self.card_ranks[3], other.card_ranks[3]))
            .then(super::cmp_card_ranks(self.card_ranks[4], other.card_ranks[4]))
    }
}

impl cmp::PartialOrd for HandValue {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn hand_rank_values_are_compared_by_rank_type_at_first() {
        let ranks_a = [
            CardRank::King,
            CardRank::Jack,
            CardRank::new(9),
            CardRank::new(7),
            CardRank::new(5),
        ];
        let ranks_b = [
            CardRank::Ace,
            CardRank::Queen,
            CardRank::new(10),
            CardRank::new(8),
            CardRank::new(6),
        ];

        let a = HandValue{ hand_rank: HandRank::Flush, card_ranks: ranks_a };
        let b = HandValue{ hand_rank: HandRank::HighCard, card_ranks: ranks_b };

        assert_eq!(Ordering::Greater, a.cmp(&b));
    }

    #[test]
    fn hand_values_of_same_rank_are_compared_by_highest_card_rank() {
        let ranks_a = [
            CardRank::new(7),
            CardRank::new(6),
            CardRank::new(5),
            CardRank::new(4),
            CardRank::new(3),
        ];
        let ranks_b = [
            CardRank::Ace,
            CardRank::new(5),
            CardRank::new(4),
            CardRank::new(3),
            CardRank::new(2),
        ];

        let a = HandValue{ hand_rank: HandRank::Flush, card_ranks: ranks_a };
        let b = HandValue{ hand_rank: HandRank::Flush, card_ranks: ranks_b };

        assert_eq!(Ordering::Less, a.cmp(&b));
    }

    #[test]
    fn hand_values_of_same_rank_can_be_compared_by_second_highest_card_rank() {
        let ranks_a = [
            CardRank::Ace,
            CardRank::new(6),
            CardRank::new(5),
            CardRank::new(4),
            CardRank::new(3),
        ];
        let ranks_b = [
            CardRank::Ace,
            CardRank::King,
            CardRank::new(4),
            CardRank::new(3),
            CardRank::new(2),
        ];

        let a = HandValue{ hand_rank: HandRank::Flush, card_ranks: ranks_a };
        let b = HandValue{ hand_rank: HandRank::Flush, card_ranks: ranks_b };

        assert_eq!(Ordering::Less, a.cmp(&b));
    }

    #[test]
    fn hand_values_of_same_rank_can_be_compared_by_third_highest_card_rank() {
        let ranks_a = [
            CardRank::Ace,
            CardRank::King,
            CardRank::new(5),
            CardRank::new(4),
            CardRank::new(3),
        ];
        let ranks_b = [
            CardRank::Ace,
            CardRank::King,
            CardRank::Queen,
            CardRank::new(3),
            CardRank::new(2),
        ];

        let a = HandValue{ hand_rank: HandRank::Flush, card_ranks: ranks_a };
        let b = HandValue{ hand_rank: HandRank::Flush, card_ranks: ranks_b };

        assert_eq!(Ordering::Less, a.cmp(&b));
    }

    #[test]
    fn hand_values_of_same_rank_can_be_compared_by_fourth_highest_card_rank() {
        let ranks_a = [
            CardRank::Ace,
            CardRank::King,
            CardRank::Queen,
            CardRank::new(4),
            CardRank::new(3),
        ];
        let ranks_b = [
            CardRank::Ace,
            CardRank::King,
            CardRank::Queen,
            CardRank::Jack,
            CardRank::new(2),
        ];

        let a = HandValue{ hand_rank: HandRank::Flush, card_ranks: ranks_a };
        let b = HandValue{ hand_rank: HandRank::Flush, card_ranks: ranks_b };

        assert_eq!(Ordering::Less, a.cmp(&b));
    }

    #[test]
    fn hand_values_of_same_rank_can_be_compared_by_lowest_card_rank() {
        let ranks_a = [
            CardRank::Ace,
            CardRank::King,
            CardRank::Queen,
            CardRank::Jack,
            CardRank::new(3),
        ];
        let ranks_b = [
            CardRank::Ace,
            CardRank::King,
            CardRank::Queen,
            CardRank::Jack,
            CardRank::new(2),
        ];

        let a = HandValue{ hand_rank: HandRank::Flush, card_ranks: ranks_a };
        let b = HandValue{ hand_rank: HandRank::Flush, card_ranks: ranks_b };

        assert_eq!(Ordering::Greater, a.cmp(&b));
    }

    #[test]
    fn hand_values_with_same_type_and_card_ranks_are_equal() {
        let ranks = [
            CardRank::Ace,
            CardRank::King,
            CardRank::Queen,
            CardRank::Jack,
            CardRank::new(2),
        ];

        let a = HandValue{ hand_rank: HandRank::Flush, card_ranks: ranks };
        let b = HandValue{ hand_rank: HandRank::Flush, card_ranks: ranks };

        assert_eq!(Ordering::Equal, a.cmp(&b));
    }
}