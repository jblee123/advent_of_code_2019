use std::str::FromStr;

use aoc2019_utils::*;

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

pub struct BigDeck { num_cards: u64 }

impl BigDeck {
    pub fn new(num_cards: u64) -> Self {
        Self { num_cards: num_cards }
    }

    fn shuffle_into_new_stack_pos(&self, card_num: u64) -> u64 {
        self.num_cards - card_num - 1
    }

    fn shuffle_cut_pos(&self, card_num: u64, n: i32) -> u64 {
        let n = if n >= 0 {
            n as u64
        } else {
            self.num_cards - (n.abs() as u64)
        };

        if card_num >= n {
            card_num - n
        } else {
            self.num_cards - n + card_num
        }
    }

    fn shuffle_with_incr_pos(&self, card_num: u64, n: u32) -> u64 {
        (card_num * n as u64) % self.num_cards
    }

    fn unshuffle_into_new_stack_pos(&self, card_num: u64) -> u64 {
        self.shuffle_into_new_stack_pos(card_num)
    }

    fn unshuffle_cut_pos(&self, card_num: u64, n: i32) -> u64 {
        self.shuffle_cut_pos(card_num, -n)
    }

    fn get_num_wraps_from_shuffle_with_incr(&self, card_num: u64, n: u64)
    -> (u64, u64) {
        let shorter_deck_len = n + (self.num_cards % n);
        let num_chunks = self.num_cards / n;
        let partial_chunk_start = num_chunks * n;
        let target_idx = if card_num >= partial_chunk_start {
            n + (card_num - partial_chunk_start)
        } else {
            card_num % n
        };

        let mut num_wraps = 0;
        let mut last_chunk_hits = 0;
        let mut pos = 0;
        while pos != target_idx {
            pos += n;
            if pos >= shorter_deck_len {
                pos -= shorter_deck_len;
                num_wraps += 1;
            }
            if pos >= n {
                last_chunk_hits += 1;
            }
        }

        (num_wraps, last_chunk_hits)
    }

    fn unshuffle_with_incr_pos(&self, card_num: u64, n: u32) -> u64 {
        let n = n as u64;

        let (num_wraps, last_chunk_hits) =
            self.get_num_wraps_from_shuffle_with_incr(card_num, n);

        let chunk_num = card_num / n;
        let num_chunks = self.num_cards / n;
        let partial_chunk_start = num_chunks * n;
        let orig_card_num =
            (num_wraps * num_chunks) + last_chunk_hits + chunk_num;
        if card_num >= partial_chunk_start {
            orig_card_num - 1
        } else {
            orig_card_num
        }
    }

    pub fn shuffle_pos(
        &self,
        shuffle_type: ShuffleType,
        card_num: u64,
    ) -> u64 {
        match shuffle_type {
            ShuffleType::IntoNewStack => self.shuffle_into_new_stack_pos(card_num),
            ShuffleType::Cut(n) => self.shuffle_cut_pos(card_num, n),
            ShuffleType::WithIncrement(n) => self.shuffle_with_incr_pos(card_num, n),
        }
    }

    pub fn shuffle_pos_multi(
        &self,
        shuffle_types: &[ShuffleType],
        card_num: u64,
    ) -> u64 {
        let mut orig_card_num = card_num;
        for shuffle_type in shuffle_types {
            orig_card_num = self.shuffle_pos(*shuffle_type, orig_card_num);
        }

        orig_card_num
    }

    pub fn unshuffle_pos(
        &self,
        shuffle_type: ShuffleType,
        card_num: u64,
    ) -> u64 {
        match shuffle_type {
            ShuffleType::IntoNewStack => self.unshuffle_into_new_stack_pos(card_num),
            ShuffleType::Cut(n) => self.unshuffle_cut_pos(card_num, n),
            ShuffleType::WithIncrement(n) => self.unshuffle_with_incr_pos(card_num, n),
        }
    }

    pub fn unshuffle_pos_multi(
        &self,
        shuffle_types: &[ShuffleType],
        card_num: u64,
    ) -> u64 {
        let mut orig_card_num = card_num;
        for shuffle_type in shuffle_types.iter().rev() {
            orig_card_num = self.unshuffle_pos(*shuffle_type, orig_card_num);
        }

        orig_card_num
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CompiledShuffle { pub a: u64, pub b: u64 }

pub struct BigDeck2 { num_cards: u64 }

impl BigDeck2 {
    pub fn new(num_cards: u64) -> Self {
        Self { num_cards: num_cards }
    }

    fn modulo(a: i64, b: i64) -> i64 {
        ((a % b) + b) % b
    }

    // Implementation from: https://en.wikipedia.org/wiki/Modular_arithmetic
    // fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    //     let mut d = 0;
    //     let mp2 = m >> 1;
    //     let mut a = a % m;
    //     let b = b % m;
    //     for _ in 0..64 {
    //         d = if d > mp2 { (d << 1) - m } else { d << 1 };
    //         if (a & 0x8000000000000000u64) != 0 {
    //             d += b;
    //         }
    //         if d >= m {
    //             d -= m;
    //         }
    //         a <<= 1;
    //     }
    //     d
    // }

    // Implementation from: https://en.wikipedia.org/wiki/Modular_arithmetic
    fn mul_mod2(a: u64, b: u64, m: u64) -> u64 {
        let a = a % m;
        let b = b % m;
        let c = (a as f64 * b as f64 / m as f64) as u64;
        let r = ((a * b - c * m) as i64) % (m as i64);
        (if r < 0 { r + (m as i64) } else { r }) as u64
    }

    fn compile_into_new_stack_shuffle(
        &self,
        shuffle: CompiledShuffle
    ) -> CompiledShuffle {
        let mut shuffle = shuffle;
        shuffle.a = Self::mul_mod2(shuffle.a, self.num_cards - 1, self.num_cards);
        shuffle.b = self.num_cards - 1 - shuffle.b;
        shuffle
    }

    fn compile_cut_shuffle(
        &self,
        n: i32,
        shuffle: CompiledShuffle,
    ) -> CompiledShuffle {
        let mut shuffle = shuffle;
        let n = if n >= 0 {
            self.num_cards - (n as u64)
        } else {
            n.abs() as u64
        };
        shuffle.b = (shuffle.b + n) % self.num_cards;
        shuffle
    }

    fn compile_with_inc_shuffle(
        &self,
        n: u32,
        shuffle: CompiledShuffle,
    ) -> CompiledShuffle {
        let mut shuffle = shuffle;
        shuffle.a = Self::mul_mod2(shuffle.a, n as u64, self.num_cards);
        shuffle.b = Self::mul_mod2(shuffle.b, n as u64, self.num_cards);
        shuffle
    }

    pub fn compile(&self, shuffles: &[ShuffleType]) -> CompiledShuffle {

        let mut shuffle = CompiledShuffle { a: 1, b: 0 };

        for shuffle_type in shuffles {
            shuffle = match shuffle_type {
                ShuffleType::IntoNewStack => {
                    self.compile_into_new_stack_shuffle(shuffle)
                },
                ShuffleType::Cut(n) => {
                    self.compile_cut_shuffle(*n, shuffle)
                },
                ShuffleType::WithIncrement(n) => {
                    self.compile_with_inc_shuffle(*n, shuffle)
                },
            }
        }

        shuffle
    }

    pub fn apply_shuffle(&self, shuffle: &CompiledShuffle, pos: u64) -> u64 {
        let ca = Self::mul_mod2(pos, shuffle.a, self.num_cards);
        (ca + shuffle.b) % self.num_cards
    }

    fn combine_shuffles(
        &self,
        shuffle1: CompiledShuffle,
        shuffle2: CompiledShuffle,
    ) -> CompiledShuffle {
        let new_a = Self::mul_mod2(shuffle1.a, shuffle2.a, self.num_cards);
        let new_b = Self::mul_mod2(shuffle1.b, shuffle2.a, self.num_cards);
        let new_b = (new_b + shuffle2.b) % self.num_cards;
        CompiledShuffle { a: new_a, b: new_b }
    }

    pub fn stack_shuffle_chunk(
        &self,
        shuffle: CompiledShuffle,
        num_stacked: u64,
    ) -> CompiledShuffle {
        if num_stacked == 0 {
            return CompiledShuffle { a: 1, b: 0 };
        }

        let mut new_shuf = shuffle.clone();
        for _ in 0..(num_stacked - 1) {
            new_shuf = self.combine_shuffles(new_shuf, shuffle);
        }
        new_shuf
    }

    pub fn stack_shuffle(
        &self,
        shuffle: &CompiledShuffle,
        num_stacked: u64,
        chunk_size: u64,
    ) -> CompiledShuffle {

        let num_chunks = num_stacked / chunk_size;
        let num_remaining = num_stacked % chunk_size;

        let mut total = self.stack_shuffle_chunk(*shuffle, num_remaining);
        if num_chunks > 0 {
            let chunk = self.stack_shuffle_chunk(*shuffle, chunk_size);
            let chunk = self.stack_shuffle_chunk(chunk, num_chunks);
            total = self.combine_shuffles(total, chunk);
        }

        total
    }

    pub fn reverse_shuffle(&self, shuffle: &CompiledShuffle, pos: u64) -> u64 {
        let pos = pos as i64 - shuffle.b as i64;

        let (s, _, _) = gcd_extended(shuffle.a as i64, self.num_cards as i64);

        let mut ans = Self::mul_mod2(
            pos.abs() as u64,
            s.abs() as u64,
            self.num_cards
        );
        if pos.is_negative() != s.is_negative() {
            ans = Self::modulo(-(ans as i64), self.num_cards as i64) as u64;
        }
        ans
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

    #[test]
    fn test_big_deck_unshuffle_into_new_stack_pos() {
        let deck = BigDeck::new(100);

        let result = deck.unshuffle_pos(ShuffleType::IntoNewStack, 0);
        assert_eq!(result, 99);

        let result = deck.unshuffle_pos(ShuffleType::IntoNewStack, 99);
        assert_eq!(result, 0);

        let result = deck.unshuffle_pos(ShuffleType::IntoNewStack, 49);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_big_deck_unshuffle_cut_pos() {
        let deck = BigDeck::new(100);

        let result = deck.unshuffle_pos(ShuffleType::Cut(10), 90);
        assert_eq!(result, 0);

        let result = deck.unshuffle_pos(ShuffleType::Cut(10), 1);
        assert_eq!(result, 11);

        let result = deck.unshuffle_pos(ShuffleType::Cut(-10), 0);
        assert_eq!(result, 90);

        let result = deck.unshuffle_pos(ShuffleType::Cut(-10), 99);
        assert_eq!(result, 89);
    }

    #[test]
    fn test_big_deck_unshuffle_with_incr_pos() {
        const SHUFFLE_WITH_INC_3: ShuffleType = ShuffleType::WithIncrement(3);
        const SHUFFLE_WITH_INC_5: ShuffleType = ShuffleType::WithIncrement(5);

        let deck = BigDeck::new(10);

        let (r1, r2) = deck.get_num_wraps_from_shuffle_with_incr(0, 3);
        assert_eq!((r1, r2), (0, 0));
        let (r1, r2) = deck.get_num_wraps_from_shuffle_with_incr(1, 3);
        assert_eq!((r1, r2), (2, 1));
        let (r1, r2) = deck.get_num_wraps_from_shuffle_with_incr(2, 3);
        assert_eq!((r1, r2), (1, 1));
        let (r1, r2) = deck.get_num_wraps_from_shuffle_with_incr(3, 3);
        assert_eq!((r1, r2), (0, 0));
        let (r1, r2) = deck.get_num_wraps_from_shuffle_with_incr(4, 3);
        assert_eq!((r1, r2), (2, 1));
        let (r1, r2) = deck.get_num_wraps_from_shuffle_with_incr(5, 3);
        assert_eq!((r1, r2), (1, 1));
        let (r1, r2) = deck.get_num_wraps_from_shuffle_with_incr(6, 3);
        assert_eq!((r1, r2), (0, 0));
        let (r1, r2) = deck.get_num_wraps_from_shuffle_with_incr(7, 3);
        assert_eq!((r1, r2), (2, 1));
        let (r1, r2) = deck.get_num_wraps_from_shuffle_with_incr(8, 3);
        assert_eq!((r1, r2), (1, 1));
        let (r1, r2) = deck.get_num_wraps_from_shuffle_with_incr(9, 3);
        assert_eq!((r1, r2), (0, 1));

        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_3, 0);
        assert_eq!(result, 0);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_3, 1);
        assert_eq!(result, 7);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_3, 2);
        assert_eq!(result, 4);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_3, 3);
        assert_eq!(result, 1);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_3, 4);
        assert_eq!(result, 8);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_3, 5);
        assert_eq!(result, 5);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_3, 6);
        assert_eq!(result, 2);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_3, 7);
        assert_eq!(result, 9);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_3, 8);
        assert_eq!(result, 6);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_3, 9);
        assert_eq!(result, 3);

        let deck = BigDeck::new(13);

        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_5, 0);
        assert_eq!(result, 0);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_5, 1);
        assert_eq!(result, 8);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_5, 2);
        assert_eq!(result, 3);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_5, 3);
        assert_eq!(result, 11);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_5, 4);
        assert_eq!(result, 6);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_5, 5);
        assert_eq!(result, 1);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_5, 6);
        assert_eq!(result, 9);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_5, 7);
        assert_eq!(result, 4);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_5, 8);
        assert_eq!(result, 12);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_5, 9);
        assert_eq!(result, 7);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_5, 10);
        assert_eq!(result, 2);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_5, 11);
        assert_eq!(result, 10);
        let result = deck.unshuffle_pos(SHUFFLE_WITH_INC_5, 12);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_big_deck_unshuffle_pos_multi() {
        let shuffles = vec![
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

        let deck = BigDeck::new(10);

        let result = deck.unshuffle_pos_multi(&shuffles, 0);
        assert_eq!(result, 9);
        let result = deck.unshuffle_pos_multi(&shuffles, 1);
        assert_eq!(result, 2);
        let result = deck.unshuffle_pos_multi(&shuffles, 2);
        assert_eq!(result, 5);
        let result = deck.unshuffle_pos_multi(&shuffles, 3);
        assert_eq!(result, 8);
        let result = deck.unshuffle_pos_multi(&shuffles, 4);
        assert_eq!(result, 1);
        let result = deck.unshuffle_pos_multi(&shuffles, 5);
        assert_eq!(result, 4);
        let result = deck.unshuffle_pos_multi(&shuffles, 6);
        assert_eq!(result, 7);
        let result = deck.unshuffle_pos_multi(&shuffles, 7);
        assert_eq!(result, 0);
        let result = deck.unshuffle_pos_multi(&shuffles, 8);
        assert_eq!(result, 3);
        let result = deck.unshuffle_pos_multi(&shuffles, 9);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_big_deck2_modulo() {
        assert_eq!(BigDeck2::modulo(8, 100), 8);
        assert_eq!(BigDeck2::modulo(8, 108), 8);
        assert_eq!(BigDeck2::modulo(-8, 100), 92);
    }

    #[test]
    fn test_big_deck2_mul_mod() {
        // assert_eq!(BigDeck2::mul_mod(4, 25, 75), 25);
        // assert_eq!(BigDeck2::mul_mod(8, 100, 75), 50);
        // assert_eq!(BigDeck2::mul_mod(8, 100, 800), 0);
        // assert_eq!(BigDeck2::mul_mod(8, 100, 1000), 800);

        assert_eq!(BigDeck2::mul_mod2(4, 25, 75), 25);
        assert_eq!(BigDeck2::mul_mod2(8, 100, 75), 50);
        assert_eq!(BigDeck2::mul_mod2(8, 100, 800), 0);
        assert_eq!(BigDeck2::mul_mod2(8, 100, 1000), 800);
    }

    #[test]
    fn test_big_deck2_compile() {
        let deck = BigDeck2::new(100);

        let result = deck.compile(&vec![
            ShuffleType::WithIncrement(13),
        ]);
        assert_eq!(
            result,
            CompiledShuffle { a: 13, b: 0 },
        );

        let result = deck.compile(&vec![
            ShuffleType::WithIncrement(13),
            ShuffleType::Cut(80),
        ]);
        assert_eq!(
            result,
            CompiledShuffle { a: 13, b: 20 },
        );

        let result = deck.compile(&vec![
            ShuffleType::WithIncrement(13),
            ShuffleType::Cut(-20),
        ]);
        assert_eq!(
            result,
            CompiledShuffle { a: 13, b: 20 },
        );

        // let target = Deck { cards: vec![8, 9, 0, 1, 2, 3, 4, 5, 6, 7] };
        let deck = BigDeck2::new(10);
        let shuffle = deck.compile(&vec![
            ShuffleType::Cut(-2),
        ]);
        assert_eq!(
            shuffle,
            CompiledShuffle { a: 1, b: 2 },
        );
        assert_eq!(deck.apply_shuffle(&shuffle, 0), 2);
        assert_eq!(deck.apply_shuffle(&shuffle, 1), 3);
        assert_eq!(deck.apply_shuffle(&shuffle, 7), 9);
        assert_eq!(deck.apply_shuffle(&shuffle, 8), 0);
        assert_eq!(deck.apply_shuffle(&shuffle, 9), 1);

        // let target = Deck { cards: vec![2, 3, 4, 5, 6, 7, 8, 9, 0, 1] };
        let deck = BigDeck2::new(10);
        let shuffle = deck.compile(&vec![
            ShuffleType::Cut(2),
        ]);
        assert_eq!(
            shuffle,
            CompiledShuffle { a: 1, b: 8 },
        );
        assert_eq!(deck.apply_shuffle(&shuffle, 0), 8);
        assert_eq!(deck.apply_shuffle(&shuffle, 1), 9);
        assert_eq!(deck.apply_shuffle(&shuffle, 9), 7);
        assert_eq!(deck.apply_shuffle(&shuffle, 2), 0);
        assert_eq!(deck.apply_shuffle(&shuffle, 3), 1);

        // let target = Deck { cards: vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9] };
        let deck = BigDeck2::new(10);
        let shuffle = deck.compile(&vec![
            ShuffleType::WithIncrement(7),
            ShuffleType::WithIncrement(9),
            ShuffleType::Cut(-2),
        ]);
        assert_eq!(
            shuffle,
            CompiledShuffle { a: 3, b: 2 },
        );
        assert_eq!(deck.apply_shuffle(&shuffle, 6), 0);
        assert_eq!(deck.apply_shuffle(&shuffle, 3), 1);
        assert_eq!(deck.apply_shuffle(&shuffle, 0), 2);
        assert_eq!(deck.apply_shuffle(&shuffle, 7), 3);
        assert_eq!(deck.apply_shuffle(&shuffle, 4), 4);
        assert_eq!(deck.apply_shuffle(&shuffle, 1), 5);
        assert_eq!(deck.apply_shuffle(&shuffle, 8), 6);
        assert_eq!(deck.apply_shuffle(&shuffle, 5), 7);
        assert_eq!(deck.apply_shuffle(&shuffle, 2), 8);
        assert_eq!(deck.apply_shuffle(&shuffle, 9), 9);

        let deck = BigDeck2::new(10);
        let shuffle = deck.compile(&vec![
            ShuffleType::IntoNewStack,
        ]);
        assert_eq!(
            shuffle,
            CompiledShuffle { a: 9, b: 9 },
        );
        assert_eq!(deck.apply_shuffle(&shuffle, 0), 9);
        assert_eq!(deck.apply_shuffle(&shuffle, 4), 5);
        assert_eq!(deck.apply_shuffle(&shuffle, 5), 4);
        assert_eq!(deck.apply_shuffle(&shuffle, 9), 0);

        // let target = Deck { cards: vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7] };
        let deck = BigDeck2::new(10);
        let shuffle = deck.compile(&vec![
            ShuffleType::WithIncrement(7),
            ShuffleType::IntoNewStack,
            ShuffleType::IntoNewStack,
        ]);
        assert_eq!(deck.apply_shuffle(&shuffle, 0), 0);
        assert_eq!(deck.apply_shuffle(&shuffle, 3), 1);
        assert_eq!(deck.apply_shuffle(&shuffle, 6), 2);
        assert_eq!(deck.apply_shuffle(&shuffle, 9), 3);
        assert_eq!(deck.apply_shuffle(&shuffle, 2), 4);
        assert_eq!(deck.apply_shuffle(&shuffle, 5), 5);
        assert_eq!(deck.apply_shuffle(&shuffle, 8), 6);
        assert_eq!(deck.apply_shuffle(&shuffle, 1), 7);
        assert_eq!(deck.apply_shuffle(&shuffle, 4), 8);
        assert_eq!(deck.apply_shuffle(&shuffle, 7), 9);

        // let target = Deck { cards: vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6] };
        let deck = BigDeck2::new(10);
        let shuffle = deck.compile(&vec![
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
        assert_eq!(deck.apply_shuffle(&shuffle, 9), 0);
        assert_eq!(deck.apply_shuffle(&shuffle, 2), 1);
        assert_eq!(deck.apply_shuffle(&shuffle, 5), 2);
        assert_eq!(deck.apply_shuffle(&shuffle, 8), 3);
        assert_eq!(deck.apply_shuffle(&shuffle, 1), 4);
        assert_eq!(deck.apply_shuffle(&shuffle, 4), 5);
        assert_eq!(deck.apply_shuffle(&shuffle, 7), 6);
        assert_eq!(deck.apply_shuffle(&shuffle, 0), 7);
        assert_eq!(deck.apply_shuffle(&shuffle, 3), 8);
        assert_eq!(deck.apply_shuffle(&shuffle, 6), 9);
    }

    #[test]
    fn test_big_deck2_stack_shuffle() {
        let deck = BigDeck2::new(100);
        let shuffle = CompiledShuffle { a: 5, b: 6 };
        let target = CompiledShuffle { a: 25, b: 86 };
        let stacked_shuffle = deck.stack_shuffle(&shuffle, 5, 2);
        assert_eq!(stacked_shuffle, target);

        const START_POS: u64 = 5;
        let mut new_pos1 = START_POS;
        new_pos1 = deck.apply_shuffle(&shuffle, new_pos1);
        new_pos1 = deck.apply_shuffle(&shuffle, new_pos1);
        new_pos1 = deck.apply_shuffle(&shuffle, new_pos1);
        new_pos1 = deck.apply_shuffle(&shuffle, new_pos1);
        new_pos1 = deck.apply_shuffle(&shuffle, new_pos1);

        let new_pos2 = deck.apply_shuffle(&stacked_shuffle, START_POS);
        assert_eq!(new_pos1, new_pos2);
    }

    #[test]
    fn test_reverse_shuffle() {
        // let target = Deck { cards: vec![0, 2, 4, 6, 8, 10, 12, 1, 3, 5, 7, 9, 11] };
        let deck = BigDeck2::new(13);
        let shuffle = deck.compile(&vec![
            ShuffleType::WithIncrement(7),
        ]);

        assert_eq!(deck.reverse_shuffle(&shuffle, 0), 0);
        assert_eq!(deck.reverse_shuffle(&shuffle, 1), 2);
        assert_eq!(deck.reverse_shuffle(&shuffle, 2), 4);
        assert_eq!(deck.reverse_shuffle(&shuffle, 3), 6);
        assert_eq!(deck.reverse_shuffle(&shuffle, 4), 8);
        assert_eq!(deck.reverse_shuffle(&shuffle, 5), 10);
        assert_eq!(deck.reverse_shuffle(&shuffle, 6), 12);
        assert_eq!(deck.reverse_shuffle(&shuffle, 7), 1);
        assert_eq!(deck.reverse_shuffle(&shuffle, 8), 3);
        assert_eq!(deck.reverse_shuffle(&shuffle, 9), 5);
        assert_eq!(deck.reverse_shuffle(&shuffle, 10), 7);
        assert_eq!(deck.reverse_shuffle(&shuffle, 11), 9);
        assert_eq!(deck.reverse_shuffle(&shuffle, 12), 11);

        // let target = Deck { cards: vec![0, 8, 3, 11, 6, 1, 9, 4, 12, 7, 2, 10, 5] };
        let deck = BigDeck2::new(13);
        let shuffle = deck.compile(&vec![
            ShuffleType::WithIncrement(5),
        ]);
        println!("shuffle: {:?}", shuffle);

        assert_eq!(deck.reverse_shuffle(&shuffle, 0), 0);
        assert_eq!(deck.reverse_shuffle(&shuffle, 1), 8);
        assert_eq!(deck.reverse_shuffle(&shuffle, 2), 3);
        assert_eq!(deck.reverse_shuffle(&shuffle, 3), 11);
        assert_eq!(deck.reverse_shuffle(&shuffle, 4), 6);
        assert_eq!(deck.reverse_shuffle(&shuffle, 5), 1);
        assert_eq!(deck.reverse_shuffle(&shuffle, 6), 9);
        assert_eq!(deck.reverse_shuffle(&shuffle, 7), 4);
        assert_eq!(deck.reverse_shuffle(&shuffle, 8), 12);
        assert_eq!(deck.reverse_shuffle(&shuffle, 9), 7);
        assert_eq!(deck.reverse_shuffle(&shuffle, 10), 2);
        assert_eq!(deck.reverse_shuffle(&shuffle, 11), 10);
        assert_eq!(deck.reverse_shuffle(&shuffle, 12), 5);

        // let target = Deck { cards: vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9] };
        let deck = BigDeck2::new(10);
        let shuffle = deck.compile(&vec![
            ShuffleType::WithIncrement(7),
            ShuffleType::WithIncrement(9),
            ShuffleType::Cut(-2),
        ]);
        assert_eq!(
            shuffle,
            CompiledShuffle { a: 3, b: 2 },
        );
        assert_eq!(deck.reverse_shuffle(&shuffle, 0), 6);
        assert_eq!(deck.reverse_shuffle(&shuffle, 1), 3);
        assert_eq!(deck.reverse_shuffle(&shuffle, 2), 0);
        assert_eq!(deck.reverse_shuffle(&shuffle, 3), 7);
        assert_eq!(deck.reverse_shuffle(&shuffle, 4), 4);
        assert_eq!(deck.reverse_shuffle(&shuffle, 5), 1);
        assert_eq!(deck.reverse_shuffle(&shuffle, 6), 8);
        assert_eq!(deck.reverse_shuffle(&shuffle, 7), 5);
        assert_eq!(deck.reverse_shuffle(&shuffle, 8), 2);
        assert_eq!(deck.reverse_shuffle(&shuffle, 9), 9);

        // let target = Deck { cards: vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6] };
        let deck = BigDeck2::new(10);
        let shuffle = deck.compile(&vec![
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
        assert_eq!(deck.reverse_shuffle(&shuffle, 0), 9);
        assert_eq!(deck.reverse_shuffle(&shuffle, 1), 2);
        assert_eq!(deck.reverse_shuffle(&shuffle, 2), 5);
        assert_eq!(deck.reverse_shuffle(&shuffle, 3), 8);
        assert_eq!(deck.reverse_shuffle(&shuffle, 4), 1);
        assert_eq!(deck.reverse_shuffle(&shuffle, 5), 4);
        assert_eq!(deck.reverse_shuffle(&shuffle, 6), 7);
        assert_eq!(deck.reverse_shuffle(&shuffle, 7), 0);
        assert_eq!(deck.reverse_shuffle(&shuffle, 8), 3);
        assert_eq!(deck.reverse_shuffle(&shuffle, 9), 6);

        let shuffle_stacked = deck.stack_shuffle(&shuffle, 7, 3);
        let shuf_pos = deck.apply_shuffle(&shuffle_stacked, 5);
        let rev_shuf_pos = deck.reverse_shuffle(&shuffle_stacked, shuf_pos);
        assert_eq!(rev_shuf_pos, 5);
    }
}
