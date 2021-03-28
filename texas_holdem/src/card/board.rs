use crate::Phase;
use super::StartingHand;
use playing_card::card::NonJokerCard;
use playing_card::deck::Deck;

pub struct Board {
    deck: Deck<NonJokerCard>,
    flop: Option<[NonJokerCard;3]>,
    turn: Option<NonJokerCard>,
    river: Option<NonJokerCard>,
}

impl Board {
    pub fn new() -> Self {
        let mut deck = Deck::new(NonJokerCard::all());
        deck.shuffle();
        Self {
            deck,
            flop: None,
            turn: None,
            river: None,
        }
    }

    pub fn flop(&self) -> Option<[NonJokerCard;3]> {
        self.flop
    }

    pub fn turn(&self) -> Option<NonJokerCard> {
        self.turn
    }

    pub fn river(&self) -> Option<NonJokerCard> {
        self.river
    }

    pub fn cards(&self) -> Vec<NonJokerCard> {
        let mut v = Vec::new();
        if let Some(cards) = self.flop {
            v.push(cards[0]);
            v.push(cards[1]);
            v.push(cards[2]);
        }
        if let Some(c) = self.turn {
            v.push(c);
        }
        if let Some(c) = self.river {
            v.push(c);
        }
        v
    }

    pub fn current_phase(&self) -> Phase {
        if self.flop.is_none() {
            Phase::PreFlop
        } else if self.turn.is_none() {
            Phase::Flop
        } else if self.river.is_none() {
            Phase::Turn
        } else {
            Phase::River
        }
    }

    pub fn deal_starting_hands(&mut self, num_of_players: usize) -> Vec<StartingHand> {
        let mut v = Vec::new();
        while v.len() < num_of_players {
            let hand = StartingHand::new(
                self.deck.deal_one().expect("Card deck has run out while dealing hand!!"),
                self.deck.deal_one().expect("Card deck has run out while dealing hand!!")
            );
            v.push(hand);
        }
        v
    }

    pub fn deal_next_card(&mut self) {
        match self.current_phase() {
            Phase::PreFlop => { 
                let cards = [
                    self.deck.deal_one().expect("Card deck has run out while dealing flop!!"),
                    self.deck.deal_one().expect("Card deck has run out while dealing flop!!"),
                    self.deck.deal_one().expect("Card deck has run out while dealing flop!!"),
                ];
                self.flop = Some(cards);
            },
            Phase::Flop => {
                let card = self.deck.deal_one()
                               .expect("Card deck has run out while dealing turn!!");
                self.turn = Some(card);
            },
            Phase::Turn => {
                let card = self.deck.deal_one()
                               .expect("Card deck has run out while dealing river!!");
                self.river = Some(card);
            }
            Phase::River => ()
        }
    }

    pub fn deal_cards_until(&mut self, until: Phase) {
        while self.current_phase() < until {
            self.deal_next_card();
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deals_hand_to_given_number_of_players() {
        let mut board = Board::new();
        let hands = board.deal_starting_hands(2);
        assert_eq!(2, hands.len());
    }

    #[test]
    fn initial_phase_is_pre_flop() {
        let board = Board::new();
        assert_eq!(Phase::PreFlop, board.current_phase());
    }

    #[test]
    fn cards_are_empty_at_first() {
        let board = Board::new();
        assert_eq!(0, board.cards().len());
    }

    #[test]
    fn flop_is_dealt_by_first_deal() {
        let mut board = Board::new();
        board.deal_next_card();
        assert!(board.flop.is_some());
        assert!(board.turn.is_none());
        assert!(board.river.is_none());
    }
 
    #[test]
    fn phase_becomes_flop_after_first_deal() {
        let mut board = Board::new();
        board.deal_next_card();
        assert_eq!(Phase::Flop, board.current_phase());
    }
 
    #[test]
    fn length_of_cards_is_3_after_first_deal() {
        let mut board = Board::new();
        board.deal_next_card();
        assert_eq!(3, board.cards().len());
    }

    #[test]
    fn turn_is_dealt_by_second_deal() {
        let mut board = Board::new();
        board.deal_next_card();
        board.deal_next_card();
        assert!(board.flop.is_some());
        assert!(board.turn.is_some());
        assert!(board.river.is_none());
    }
 
    #[test]
    fn phase_becomes_turn_after_second_deal() {
        let mut board = Board::new();
        board.deal_next_card();
        board.deal_next_card();
        assert_eq!(Phase::Turn, board.current_phase());
    }
 
    #[test]
    fn length_of_cards_is_4_after_second_deal() {
        let mut board = Board::new();
        board.deal_next_card();
        board.deal_next_card();
        assert_eq!(4, board.cards().len());
    }

    #[test]
    fn river_is_dealt_by_third_deal() {
        let mut board = Board::new();
        board.deal_next_card();
        board.deal_next_card();
        board.deal_next_card();
        assert!(board.flop.is_some());
        assert!(board.turn.is_some());
        assert!(board.river.is_some());
    }
 
    #[test]
    fn phase_becomes_river_after_third_deal() {
        let mut board = Board::new();
        board.deal_next_card();
        board.deal_next_card();
        board.deal_next_card();
        assert_eq!(Phase::River, board.current_phase());
    }
 
    #[test]
    fn length_of_cards_is_5_after_third_deal() {
        let mut board = Board::new();
        board.deal_next_card();
        board.deal_next_card();
        board.deal_next_card();
        assert_eq!(5, board.cards().len());
    }

    #[test]
    fn deal_until_flop() {
        let mut board = Board::new();
        let phase = Phase::Flop;
        board.deal_cards_until(phase);
        assert_eq!(phase, board.current_phase());
        assert!(board.flop().is_some());
    }

    #[test]
    fn deal_until_turn() {
        let mut board = Board::new();
        let phase = Phase::Turn;
        board.deal_cards_until(phase);
        assert_eq!(phase, board.current_phase());
        assert!(board.turn().is_some());
    }

    #[test]
    fn deal_until_river() {
        let mut board = Board::new();
        let phase = Phase::River;
        board.deal_cards_until(phase);
        assert_eq!(phase, board.current_phase());
        assert!(board.river().is_some());
    }
}