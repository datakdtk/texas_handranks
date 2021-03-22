use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Deck<T> { card_vec: Vec<T> }

impl<T> Deck<T> {
    pub fn new(cards: Vec<T>) -> Self
    {
        Self{ card_vec: cards }
    }

    /// All cards in the deck.
    /// The first element will be dealt next.
    pub fn cards(&self) -> &[T]
    {
        &self.card_vec
    }

    /// Randomize card order.
    pub fn shuffle(&mut self)
    {
        let mut rng = rand::thread_rng();
        self.card_vec.shuffle(&mut rng);
    }

    /// Remove a card from the deck top and return it.
    /// Returns none if no card is left in the deck. 
    pub fn deal_one(&mut self) -> Option<T>
    {
        if self.card_vec.is_empty() {
            None
        } else {
            Some(self.card_vec.remove(0))
        }
    }

    /// Remove given number of cards from the deck top and return them.
    /// Size of returned value may be less than given size if the deck does not have enough cards.
    pub fn deal_many(&mut self, size: usize) -> Vec<T>
    {
        let mut dealt = Vec::new();
        while dealt.len() < size {
            let card = self.deal_one();
            match card {
                Some(x) => dealt.push(x),
                None => break,
            }
        };
        dealt
    }

    /// Remove all cards that match given condition and return them.
    pub fn search<F: Fn(&T) -> bool>(&mut self, condition: F) -> Vec<T>
    {
        let mut found = Vec::new();
        let mut i = 0;
        while i != self.card_vec.len() {
            if condition(&self.card_vec[i]) {
                let removed = self.card_vec.remove(i);
                found.push(removed);
            } else {
                i += 1;
            }
        }
        found
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn card_order_is_changed_by_shuffle() {
        let items = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let mut deck = Deck::new(items.clone());
        deck.shuffle();
        if items == deck.cards() {
            deck.shuffle(); // might get same order at random. try again;
        }
        assert_ne!(items, deck.cards());
    }

    #[test]
    fn deal_one_returns_first_item() {
        let items = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let mut deck = Deck::new(items);
        let dealt = deck.deal_one();
        assert_eq!(10, dealt.unwrap());
    }

    #[test]
    fn deal_one_returns_second_item_when_called_twice() {
        let items = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let mut deck = Deck::new(items);
        let _ = deck.deal_one();
        let dealt = deck.deal_one();
        assert_eq!(20, dealt.unwrap());
    }

    #[test]
    fn deal_one_returns_none_when_deck_is_empty() {
        let mut deck: Deck<u8> = Deck::new(Vec::new());
        let dealt = deck.deal_one();
        assert_eq!(None, dealt);
    }

    #[test]
    fn deal_one_removes_returned_item() {
        let items = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let mut deck = Deck::new(items);
        let _ = deck.deal_one();
        assert!(!deck.cards().contains(&10));
    }

    #[test]
    fn deal_many_returns_first_items() {
        let items = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let mut deck = Deck::new(items);
        let dealt = deck.deal_many(3);
        assert_eq!(vec![10, 20, 30], dealt);
    }

    #[test]
    fn deal_many_returns_preceding_items_when_called_twice() {
        let items = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let mut deck = Deck::new(items);
        let _ = deck.deal_many(3);
        let dealt = deck.deal_many(2);
        assert_eq!(vec![40, 50], dealt);
    }

    #[test]
    fn deal_many_returns_all_items_when_it_does_not_have_enough_card() {
        let items = vec![90, 100];
        let mut deck = Deck::new(items);
        let dealt = deck.deal_many(3);
        assert_eq!(vec![90, 100], dealt);
    }

    #[test]
    fn deal_many_removes_returned_items() {
        let items = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let mut deck = Deck::new(items);
        let _ = deck.deal_many(3);
        assert_eq!(&vec![40, 50, 60, 70, 80, 90, 100], deck.cards());
    }

    #[test]
    fn search_returns_matched_cards() {
        let items = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let mut deck = Deck::new(items);
        let found = deck.search(|x| x % 30 == 0);
        assert_eq!(vec![30, 60, 90], found);
    }

    #[test]
    fn search_removes_matched_cards() {
        let items = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let mut deck = Deck::new(items);
        let _ = deck.search(|x| x % 30 == 0);
        assert_eq!(vec![10, 20, 40, 50, 70, 80, 100], deck.cards());
    }
}