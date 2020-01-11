pub mod day22_utils;

use day22_utils::*;

fn main() {
    const DECK_SIZE: u32 = 10007;
    const TARGET_CARD: u32 = 2019;

    let input = aoc2019_utils::get_input("inputs/day22.txt");
    let shuffle_types = parse_input(&input);
    let mut deck = Deck::new(DECK_SIZE);
    deck.shuffle_multi(&shuffle_types);
    let card_pos = deck.find_card_position(TARGET_CARD);
    println!("card position: {}", card_pos);
}
