use std::fmt;
use std::char;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum CardRank {
    Ace,
    King,
    Queen,
    Jack,
    Number(CardRankNumber),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct CardRankNumber { value: u8 }

impl CardRank {
    pub fn new(value: u8) -> Self {
        match value {
            1 => CardRank::Ace,
            13 => CardRank::King,
            12 => CardRank::Queen,
            11 => CardRank::Jack,
            2 ..= 10 => CardRank::Number(CardRankNumber {value}),
            _ => panic!("Argument is out of range".to_string()),
        }
    }

    pub fn all() -> [Self; 13] {
        [ 
            Self::new(1),
            Self::new(2),
            Self::new(3),
            Self::new(4),
            Self::new(5),
            Self::new(6),
            Self::new(7),
            Self::new(8),
            Self::new(9),
            Self::new(10),
            Self::new(11),
            Self::new(12),
            Self::new(13),
        ]
    }

    pub fn to_int(self) -> u8 {
        match self {
            Self::Ace => 1,
            Self::King => 13,
            Self::Queen => 12,
            Self::Jack => 11,
            Self::Number(n) => n.value,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Self::Ace => 'A',
            Self::King => 'K',
            Self::Queen => 'Q',
            Self::Jack => 'J',
            Self::Number(n) if n.value == 10 => 'T',
            Self::Number(n) => char::from_digit(n.value as u32, 10).unwrap(),
        }
    }

    pub fn is_ace(self) -> bool {
        match self {
            CardRank::Ace => true,
            _ => false,
        }
    }

    pub fn is_picture_card(self) -> bool {
        match self {
            Self::King | Self::Queen | Self::Jack => true,
            _ => false,
        }
    }
}

impl fmt::Display for CardRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) if n.value == 10 => write!(f, "{}", 10),
            _ => write!(f, "{}", self.to_char()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ace_is_created_from_1() {
        assert_eq!(CardRank::new(1), CardRank::Ace);
    }

    #[test]
    fn king_is_created_from_13() {
        assert_eq!(CardRank::new(13), CardRank::King);
    }

    #[test]
    fn queen_is_created_from_12() {
        assert_eq!(CardRank::new(12), CardRank::Queen);
    }

    #[test]
    fn jack_is_created_from_11() {
        assert_eq!(CardRank::new(11), CardRank::Jack);
    }

    #[test]
    fn rank_can_create_from_10() {
        assert_eq!(
            CardRank::Number(CardRankNumber{ value: 10 }),
            CardRank::new(10),
        );
    }

    #[test]
    fn rank_can_create_from_2() {
        assert_eq!(
            CardRank::Number(CardRankNumber{ value: 2 }),
            CardRank::new(2),
        );
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn rank_can_not_create_from_0() {
        CardRank::new(0);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn rank_can_not_create_from_14() {
        CardRank::new(14);
    }

    #[test]
    fn ace_is_1_in_int() {
        assert_eq!(1, CardRank::Ace.to_int());
    }

    #[test]
    fn king_is_13_in_int() {
        assert_eq!(13, CardRank::King.to_int());
    }

    #[test]
    fn queen_is_12_in_int() {
        assert_eq!(12, CardRank::Queen.to_int());
    }

    #[test]
    fn jack_is_11_in_int() {
        assert_eq!(11, CardRank::Jack.to_int());
    }

    #[test]
    fn rank_10_is_10_in_int() {
        let n = 10;
        let rank = CardRank::Number(CardRankNumber { value: n });
        assert_eq!(n, rank.to_int());
    }

    #[test]
    fn rank_2_is_2_in_int() {
        let n = 2;
        let rank = CardRank::Number(CardRankNumber { value: n });
        assert_eq!(n, rank.to_int());
    }

    #[test]
    #[allow(non_snake_case)]
    fn ace_is_A_in_char() {
        assert_eq!('A', CardRank::Ace.to_char());
    }

    #[test]
    #[allow(non_snake_case)]
    fn king_is_K_in_char() {
        assert_eq!('K', CardRank::King.to_char());
    }

    #[test]
    #[allow(non_snake_case)]
    fn queen_is_Q_in_char() {
        assert_eq!('Q', CardRank::Queen.to_char());
    }

    #[test]
    #[allow(non_snake_case)]
    fn jack_is_J_in_char() {
        assert_eq!('J', CardRank::Jack.to_char());
    }

    #[test]
    #[allow(non_snake_case)]
    fn rank_10_is_T_in_char() {
        let n = 10;
        let rank = CardRank::Number(CardRankNumber { value: n });
        assert_eq!('T', rank.to_char());
    }

    #[test]
    fn rank_9_is_9_in_char() {
        let n = 9;
        let rank = CardRank::Number(CardRankNumber { value: n });
        assert_eq!('9', rank.to_char());
    }

    #[test]
    fn rank_2_is_2_in_char() {
        let n = 2;
        let rank = CardRank::Number(CardRankNumber { value: n });
        assert_eq!('2', rank.to_char());
    }

    #[test]
    fn rank_10_is_10_in_string() {
        let n = 10;
        let rank = CardRank::Number(CardRankNumber { value: n });
        assert_eq!("10", rank.to_string());
    }

    #[test]
    #[allow(non_snake_case)]
    fn ace_is_A_in_string() {
        assert_eq!("A", CardRank::Ace.to_string());
    }

    #[test]
    fn rank_2_is_2_in_string() {
        let n = 2;
        let rank = CardRank::Number(CardRankNumber { value: n });
        assert_eq!("2", rank.to_string());
    }

    #[test]
    fn all_includes_from_1_to_13() {
        let expected: Vec<u8> = (1 ..= 13).collect();
        let actual: Vec<u8> = CardRank::all().iter().map(|r| r.to_int()).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn ace_is_ace() {
        assert!(CardRank::Ace.is_ace())
    }

    #[test]
    fn king_is_not_ace() {
        assert!(!CardRank::King.is_ace())
    }

    #[test]
    fn rank_10_is_not_ace() {
        let n = 10;
        let rank = CardRank::Number(CardRankNumber { value: n });
        assert!(!rank.is_ace());
    }

    #[test]
    fn rank_2_is_not_ace() {
        let n = 2;
        let rank = CardRank::Number(CardRankNumber { value: n });
        assert!(!rank.is_ace());
    }

    #[test]
    fn ace_is_not_picture_card() {
        assert!(!CardRank::Ace.is_picture_card())
    }

    #[test]
    fn king_is_picture_card() {
        assert!(CardRank::King.is_picture_card())
    }

    #[test]
    fn queen_is_picture_card() {
        assert!(CardRank::Queen.is_picture_card())
    }

    #[test]
    fn jack_is_picture_card() {
        assert!(CardRank::Jack.is_picture_card())
    }

    #[test]
    fn rank_10_is_not_picture_card() {
        let n = 2;
        let rank = CardRank::Number(CardRankNumber { value: n });
        assert!(!rank.is_picture_card());
    }

    #[test]
    fn rank_2_is_not_picture_card() {
        let n = 2;
        let rank = CardRank::Number(CardRankNumber { value: n });
        assert!(!rank.is_picture_card());
    }
}