#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub enum Suit {
    Spade,
    Diamond,
    Heart,
    Club,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SuitColor {
    Black,
    Red,
}

impl Suit {
    pub fn all() -> [Suit; 4] {
        [
            Suit::Spade,
            Suit::Diamond,
            Suit::Heart,
            Suit::Club,
        ]
    }

    pub fn color(self) -> SuitColor {
        match self {
            Suit::Club | Suit::Spade => SuitColor::Black,
            Suit::Diamond | Suit::Heart => SuitColor::Red,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Suit::Club => '\u{2663}',
            Suit::Spade => '\u{2660}',
            Suit::Diamond => '\u{2666}',
            Suit::Heart => '\u{2665}',
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spade_is_black() {
        assert_eq!(Suit::Spade.color(), SuitColor::Black)
    }

    #[test]
    fn club_is_black() {
        assert_eq!(Suit::Club.color(), SuitColor::Black)
    }

    #[test]
    fn diamond_is_red() {
        assert_eq!(Suit::Diamond.color(), SuitColor::Red)
    }

    #[test]
    fn heart_is_red() {
        assert_eq!(Suit::Heart.color(), SuitColor::Red)
    }
}