use std::fmt;
use std::char;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CardRank {
    Ace,
    King,
    Queen,
    Jack,
    Number(CardRankNumber),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct CardRankNumber { value: u8 }

impl CardRank {
    pub fn new(value: u8) -> Result<Self, String> {
        match value {
            1 => Ok(CardRank::Ace),
            13 => Ok(CardRank::King),
            12 => Ok(CardRank::Queen),
            11 => Ok(CardRank::Jack),
            2 ..= 10 => Ok(CardRank::Number(CardRankNumber {value})),
            _ => Err("Argument is out of range".to_string()),
        }
    }

    pub fn all() -> [Self; 13] {
        [ 
            Self::new(1).unwrap(),
            Self::new(2).unwrap(),
            Self::new(3).unwrap(),
            Self::new(4).unwrap(),
            Self::new(5).unwrap(),
            Self::new(6).unwrap(),
            Self::new(7).unwrap(),
            Self::new(8).unwrap(),
            Self::new(9).unwrap(),
            Self::new(10).unwrap(),
            Self::new(11).unwrap(),
            Self::new(12).unwrap(),
            Self::new(13).unwrap(),
        ]
    }

    pub fn to_int(&self) -> u8 {
        match self {
            Self::Ace => 1,
            Self::King => 13,
            Self::Queen => 12,
            Self::Jack => 11,
            Self::Number(n) => n.value,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Ace => 'A',
            Self::King => 'K',
            Self::Queen => 'Q',
            Self::Jack => 'J',
            Self::Number(n) if n.value == 10 => 'T',
            Self::Number(n) => char::from_digit(n.value as u32, 10).unwrap(),
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
        assert_eq!(CardRank::new(1).unwrap(), CardRank::Ace);
    }

    #[test]
    fn king_is_created_from_13() {
        assert_eq!(CardRank::new(13).unwrap(), CardRank::King);
    }

    #[test]
    fn queen_is_created_from_12() {
        assert_eq!(CardRank::new(12).unwrap(), CardRank::Queen);
    }

    #[test]
    fn jack_is_created_from_11() {
        assert_eq!(CardRank::new(11).unwrap(), CardRank::Jack);
    }

    #[test]
    fn rank_can_create_from_10() {
        assert_eq!(
            CardRank::Number(CardRankNumber{ value: 10 }),
            CardRank::new(10).unwrap(),
        );
    }

    #[test]
    fn rank_can_create_from_2() {
        assert_eq!(
            CardRank::Number(CardRankNumber{ value: 2 }),
            CardRank::new(2).unwrap(),
        );
    }

    #[test]
    fn rank_can_not_create_from_0() {
        assert!(!CardRank::new(0).is_ok());
    }

    #[test]
    fn rank_can_not_create_from_14() {
        assert!(!CardRank::new(14).is_ok());
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
}