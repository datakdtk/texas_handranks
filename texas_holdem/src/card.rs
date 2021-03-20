use playing_card::card::CardRank;
use playing_card::card::NonJokerCard;
use playing_card::card::Suit;
use std::cmp;
use std::cmp::Ordering;

pub fn cmp_card_ranks(a: CardRank, b: CardRank) -> Ordering {
    if a.is_ace() && b.is_ace() {
        Ordering::Equal
    } else if a.is_ace() {
        Ordering::Greater
    } else if b.is_ace() {
        Ordering::Less
    } else {
        a.to_int().cmp(&b.to_int())
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct PokerCard(NonJokerCard);

impl PokerCard {
    pub fn rank(self) -> CardRank {
        self.0.rank()
    }

    pub fn suit(self) -> Suit {
        self.0.suit()
    }
}

impl cmp::Ord for PokerCard {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_card_ranks(self.rank(), other.rank())
            // Comparing suits is required for determining the unique order of hand cards,
            // that will make comparing and hashing hands easy.
            .then(self.suit().cmp(&other.suit()))
    }
}

impl cmp::PartialOrd for PokerCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ace_is_equal_to_ace() {
        assert_eq!(Ordering::Equal, cmp_card_ranks(CardRank::Ace, CardRank::Ace))
    }

    #[test]
    fn ace_is_greater_than_king() {
        assert_eq!(Ordering::Greater, cmp_card_ranks(CardRank::Ace, CardRank::King))
    }

    #[test]
    fn king_is_less_than_ace() {
        assert_eq!(Ordering::Less, cmp_card_ranks(CardRank::King, CardRank::Ace))
    }

    #[test]
    fn king_is_equal_to_king() {
        assert_eq!(Ordering::Equal, cmp_card_ranks(CardRank::King, CardRank::King))
    }

    #[test]
    fn ace_is_greater_than_2() {
        assert_eq!(Ordering::Greater, cmp_card_ranks(CardRank::Ace, CardRank::new(2).unwrap()))
    }

    #[test]
    fn rank_2_is_less_than_ace() {
        assert_eq!(Ordering::Less, cmp_card_ranks(CardRank::new(2).unwrap(), CardRank::Ace))
    }

    #[test]
    fn cards_are_compared_by_rank_at_first() {
        let mut cards = [
            PokerCard(NonJokerCard::new(Suit::Heart, CardRank::King)),
            PokerCard(NonJokerCard::new(Suit::Club, CardRank::Ace)),
            PokerCard(NonJokerCard::new(Suit::Diamond, CardRank::Jack)),
            PokerCard(NonJokerCard::new(Suit::Spade, CardRank::Queen)),
        ];
        cards.sort();
        let ranks: Vec<CardRank> = cards.iter().map(|c| c.rank()).collect();
        let expected = vec![
            CardRank::Jack,
            CardRank::Queen,
            CardRank::King,
            CardRank::Ace,
        ];
        assert_eq!(expected, ranks);
    }

    #[test]
    fn same_rank_cards_are_compared_by_suit() {
        let c1 = PokerCard(NonJokerCard::new(Suit::Heart, CardRank::Ace));
        let c2 = PokerCard(NonJokerCard::new(Suit::Spade, CardRank::Ace));
        assert_ne!(Ordering::Equal, c1.cmp(&c2));
    }

    #[test]
    fn same_rank_and_same_suit_cards_are_equal() {
        let c1 = PokerCard(NonJokerCard::new(Suit::Heart, CardRank::Ace));
        let c2 = PokerCard(NonJokerCard::new(Suit::Heart, CardRank::Ace));
        assert_eq!(Ordering::Equal, c1.cmp(&c2));
        assert_eq!(c1, c2);
    }
}


