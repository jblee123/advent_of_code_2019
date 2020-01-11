use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ShuffleType {
    IntoNewStack,
    Cut(i32),
    WithIncrement(u32),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Deck {
    pub cards: Vec<u32>,
}

impl Deck {
    pub fn new(size: u32) -> Self {
        let mut cards = Vec::with_capacity(size as usize);
        (0..size).for_each(|i| { cards.push(i) });
        Self {
            cards: cards
        }
    }

    fn shuffle_into_new_stack(&mut self, scratch: &mut Vec<u32>) {
        scratch.clear();
        for card in self.cards.iter().rev() {
            scratch.push(*card);
        }
        std::mem::swap(&mut self.cards, scratch);
    }

    fn shuffle_cut(&mut self, scratch: &mut Vec<u32>, n: i32) {
        let num_cards = self.cards.len();
        let n = if n >= 0 {
            n as usize
        } else {
            num_cards - (n.abs() as usize)
        };
        scratch.clear();
        scratch.extend_from_slice(&self.cards[n..num_cards]);
        scratch.extend_from_slice(&self.cards[0..n]);
        std::mem::swap(&mut self.cards, scratch);
    }

    fn shuffle_with_incr(&mut self, scratch: &mut Vec<u32>, n: u32) {
        let n = n as usize;
        let mut i = 0;
        for card in &self.cards {
            scratch[i] = *card;
            i = (i + n) % self.cards.len();
        }
        std::mem::swap(&mut self.cards, scratch);
    }

    fn do_shuffle(
        &mut self,
        shuffle_type: ShuffleType,
        mut scratch: &mut Vec<u32>)
    {
        match shuffle_type {
            ShuffleType::IntoNewStack => self.shuffle_into_new_stack(&mut scratch),
            ShuffleType::Cut(n) => self.shuffle_cut(&mut scratch, n),
            ShuffleType::WithIncrement(n) => self.shuffle_with_incr(&mut scratch, n),
        }
    }

    pub fn shuffle(&mut self, shuffle_type: ShuffleType) {
        let mut scratch = vec![0; self.cards.len()];
        self.do_shuffle(shuffle_type, &mut scratch);
    }

    pub fn shuffle_multi(&mut self, shuffle_types: &[ShuffleType]) {
        let mut scratch = vec![0; self.cards.len()];
        for shuffle_type in shuffle_types {
            self.do_shuffle(*shuffle_type, &mut scratch);
        }
    }

    pub fn find_card_position(&self, target: u32) -> u32 {
        self.cards.iter().position(|&card| card == target).unwrap() as u32
    }
}

pub fn parse_input(input: &str) -> Vec<ShuffleType> {
    input.lines().map(|line| {
        if line == "deal into new stack" {
            ShuffleType::IntoNewStack
        } else if line.starts_with("cut ") {
            let n_token = line.split(" ").nth(1).unwrap();
            let n = i32::from_str(n_token).unwrap();
            ShuffleType::Cut(n)
        } else if line.starts_with("deal with increment ") {
            let n_token = line.split(" ").nth(3).unwrap();
            let n = u32::from_str(n_token).unwrap();
            ShuffleType::WithIncrement(n)
        } else {
            panic!("bad line: {}", line);
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_4: &str = concat!(
        "deal into new stack\n",
        "cut -2\n",
        "deal with increment 7\n",
        "cut 8\n",
        "cut -4\n",
        "deal with increment 7\n",
        "cut 3\n",
        "deal with increment 9\n",
        "deal with increment 3\n",
        "cut -1\n",
    );

    #[test]
    fn test_parse_input() {
        let target = vec![
            ShuffleType::IntoNewStack,
            ShuffleType::Cut(-2),
            ShuffleType::WithIncrement(7),
            ShuffleType::Cut(8),
            ShuffleType::Cut(-4),
            ShuffleType::WithIncrement(7),
            ShuffleType::Cut(3),
            ShuffleType::WithIncrement(9),
            ShuffleType::WithIncrement(3),
            ShuffleType::Cut(-1),
        ];
        let result = parse_input(SAMPLE_INPUT_4);
        assert_eq!(result, target);
    }

    #[test]
    fn test_deck_new() {
        let target = Deck { cards: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9] };
        let result = Deck::new(10);
        assert_eq!(result, target);
    }

    #[test]
    fn test_deck_shuffle_into_new() {
        let target = Deck { cards: vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0] };
        let mut deck = Deck::new(10);
        deck.shuffle(ShuffleType::IntoNewStack);
        assert_eq!(deck, target);
    }

    #[test]
    fn test_deck_shuffle_cut() {
        let target = Deck { cards: vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2] };
        let mut deck = Deck::new(10);
        deck.shuffle(ShuffleType::Cut(3));
        assert_eq!(deck, target);

        let target = Deck { cards: vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5] };
        let mut deck = Deck::new(10);
        deck.shuffle(ShuffleType::Cut(-4));
        assert_eq!(deck, target);
    }

    #[test]
    fn test_deck_shuffle_with_incr() {
        let target = Deck { cards: vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3] };
        let mut deck = Deck::new(10);
        deck.shuffle(ShuffleType::WithIncrement(3));
        assert_eq!(deck, target);
    }

    #[test]
    fn test_deck_shuffle_multi() {
        let target = Deck { cards: vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7] };
        let mut deck = Deck::new(10);
        deck.shuffle_multi(&vec![
            ShuffleType::WithIncrement(7),
            ShuffleType::IntoNewStack,
            ShuffleType::IntoNewStack,
        ]);
        assert_eq!(deck, target);

        let target = Deck { cards: vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6] };
        let mut deck = Deck::new(10);
        deck.shuffle_multi(&vec![
            ShuffleType::Cut(6),
            ShuffleType::WithIncrement(7),
            ShuffleType::IntoNewStack,
        ]);
        assert_eq!(deck, target);

        let target = Deck { cards: vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9] };
        let mut deck = Deck::new(10);
        deck.shuffle_multi(&vec![
            ShuffleType::WithIncrement(7),
            ShuffleType::WithIncrement(9),
            ShuffleType::Cut(-2),
        ]);
        assert_eq!(deck, target);

        let target = Deck { cards: vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6] };
        let mut deck = Deck::new(10);
        deck.shuffle_multi(&vec![
            ShuffleType::IntoNewStack,
            ShuffleType::Cut(-2),
            ShuffleType::WithIncrement(7),
            ShuffleType::Cut(8),
            ShuffleType::Cut(-4),
            ShuffleType::WithIncrement(7),
            ShuffleType::Cut(3),
            ShuffleType::WithIncrement(9),
            ShuffleType::WithIncrement(3),
            ShuffleType::Cut(-1),
        ]);
        assert_eq!(deck, target);
    }

    #[test]
    fn test_deck_find_card_position() {
        let mut deck = Deck::new(10);
        deck.shuffle(ShuffleType::IntoNewStack);
        let result = deck.find_card_position(7);
        assert_eq!(result, 2);
    }
}
