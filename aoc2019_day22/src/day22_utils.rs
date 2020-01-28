use std::str::FromStr;

use aoc2019_utils::*;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ShuffleType {
    IntoNewStack,
    Cut(i32),
    WithIncrement(u32),
}

/// This deck is for part A. It's a straight-up simulation. Quick and simple.
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

/// A "compiled" shuffle. This class represents an equation of the form
/// c' = (cA + B)%N. The different shuffles can all be represented by
/// equations of this form, where c is the starting position of a card, N is the
/// number of cards in the stack, c' is the ending position after the shuffle is
/// compiled, and A and B are shuffle-dependent parameters. Since we're dealing
/// with such large numbers, the "cA" portion must always be a modulo
/// multiplication that will not overflow (see mul_mod and mul_mod2 above), so
/// the general equation can be written more safely as f(c) = ((cA)%N + B)%N.
///
/// Shuffles of this form can be composed, so if an existing shuffle S1(c) has
/// form S1(c) = ((cA1)%N + B1)%N, and a new shuffle S2 has form
/// S2(c) = ((cA2)%N + B2)%N, then shuffling S1 and then S2 can be written as
/// the composition of those two functions:
/// S2(S1(c)) = (([((cA1)%N + B1)%N]A2)%N + B2)%N, which can be simplified down
/// to the original form:
/// S2(S1(c)) = ((cA3)%N + B3)%N
/// Since composing two shuffles of the same form results in a third shuffle of
/// the same form as the first two, that means we can compose together as many
/// as we like, be it a list of separate shuffles or repeating the same shuffle
/// over and over again.
///
/// A no-op shuffle has the form c*1 + 0.
///
/// A reverse shuffle will be c' = (N-1) - c. Composing that into an existing
/// shuffle will be:
/// c' = (N - 1) - [((c*A)%N + B%N) % N], which simplifies to:
/// c' = [(c*A*(N-1)) + (N - 1 - B))] % N
///
/// An increment(n) shuffle has the form c' = (c*n)%N. Composing that into an
/// existing shuffle will be:
/// c' = ([((c*A)%N + B%N) % N]*n)%N, which simplifies to:
/// c' = ((c*A*n)%N + (B*n)%N) % N
///
/// A cut(n) shuffle has the form c' = (c + (N-n))%N. Composing that into an
/// existing shuffle will be:
/// c' = ([((c*A)%N + B%N) % N] + (N-n))%N, which simplifies to:
/// c' = ((c*A)%N + (N - n + B)%N) % N
///
/// A cut(-n) shuffle is equivalent to cut(N-n), and its composition with an
/// existing shuffle ends up simplifying to:
/// c' = ((c*A)%N + (n + B)%N) % N
///
/// Composing two generic shuffles
/// S1 = (c*A1 + B1)%N, and
/// S2 = (c*A2 + B2)%N results in
/// S2(S1) = [((c*A1*A2)%N + (B1*A2 + B2)%N)]%N
///
/// It occurs to me now I could have just used the simpler, stand-alone forms
/// of the shuffle types and then composed them using the generic formula
/// instead of using separate simplification code each specific shuffle, but
/// what's done is done.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CompiledShuffle { pub a: u64, pub b: u64 }

/// This deck is for part B. It does not simulate the entire deck, but works by
/// compiling different shuffles together so multiple shuffles can be applied
/// at once to a card.
pub struct BigDeck2 { num_cards: u64 }

impl BigDeck2 {
    pub fn new(num_cards: u64) -> Self {
        Self { num_cards: num_cards }
    }

    fn modulo(&self, a: i64) -> i64 {
        modulo(a, self.num_cards as i64)
    }

    fn mul_mod(&self, a: u64, b: u64) -> u64 {
        mul_mod2(a, b, self.num_cards)
    }

    fn compile_into_new_stack_shuffle(
        &self,
        shuffle: CompiledShuffle
    ) -> CompiledShuffle {
        let mut shuffle = shuffle;
        shuffle.a = self.mul_mod(shuffle.a, self.num_cards - 1);
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
        shuffle.a = self.mul_mod(shuffle.a, n as u64);
        shuffle.b = self.mul_mod(shuffle.b, n as u64);
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
        let ca = self.mul_mod(pos, shuffle.a);
        (ca + shuffle.b) % self.num_cards
    }

    fn combine_shuffles(
        &self,
        shuffle1: CompiledShuffle,
        shuffle2: CompiledShuffle,
    ) -> CompiledShuffle {
        let new_a = self.mul_mod(shuffle1.a, shuffle2.a);
        let new_b = self.mul_mod(shuffle1.b, shuffle2.a);
        let new_b = (new_b + shuffle2.b) % self.num_cards;
        CompiledShuffle { a: new_a, b: new_b }
    }

    pub fn stack_shuffle_chunk(
        &self,
        shuffle: CompiledShuffle,
        num_stacked: u64,
    ) -> CompiledShuffle {
        // Throw in this guy so that if either num_chunks or num_remaining in
        // stack_shuffle() is zero, that zero can safely be passed to this
        // function, which will result in the identity shuffle, which will not
        // have an effect on the overall shuffle stack in stack_shuffle().
        if num_stacked == 0 {
            return CompiledShuffle { a: 1, b: 0 };
        }

        let mut new_shuf = shuffle.clone();
        for _ in 0..(num_stacked - 1) {
            new_shuf = self.combine_shuffles(new_shuf, shuffle);
        }
        new_shuf
    }

    // This guy is used to stack a given shuffle num_stacked times. It's meant
    // for num_stacked to be *large* -- generally too large to just iterative
    // combine them all one by one because it would take too long, so this
    // also accepts a chunk_size parameter that is multiple orders of magnitude
    // large, but small enough to finish in a reasonable amount of time. The
    // entire stack can then be created by stacking a single chunk in a
    // reasonable amount of time and then stacking that together
    // (num_stacked DIV chunk_size) times and then adding another smaller chunk
    // of size (num_stacked MOD chunk_size) (which should also be calculable in
    // a reasonable amount of time since it will be smaller than chunk_size).
    // The theory is that if combining a shuffle 1e14 times takes too long,
    // then 1e8 iterations + 1e6 iterations can do the work of 1e14 iterations
    // in 1/(1e8)th the time.
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

        let mut ans = self.mul_mod(pos.abs() as u64, s.abs() as u64);
        if pos.is_negative() != s.is_negative() {
            ans = self.modulo(-(ans as i64)) as u64;
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
    fn test_modulo() {
        assert_eq!(modulo(8, 100), 8);
        assert_eq!(modulo(8, 108), 8);
        assert_eq!(modulo(-8, 100), 92);
    }

    #[test]
    fn test_mul_mod() {
        // assert_eq!(mul_mod(4, 25, 75), 25);
        // assert_eq!(mul_mod(8, 100, 75), 50);
        // assert_eq!(mul_mod(8, 100, 800), 0);
        // assert_eq!(mul_mod(8, 100, 1000), 800);

        assert_eq!(mul_mod2(4, 25, 75), 25);
        assert_eq!(mul_mod2(8, 100, 75), 50);
        assert_eq!(mul_mod2(8, 100, 800), 0);
        assert_eq!(mul_mod2(8, 100, 1000), 800);
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
