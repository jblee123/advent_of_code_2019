pub mod day22_utils;

// use aoc2019_utils::*;
use day22_utils::*;

fn do_sanity_check(shuffle_types: &[ShuffleType]) {
    const DECK_SIZE: u32 = 10007;
    const TARGET_CARD: u32 = 2019;

    let target_card_end_pos = {
        let mut deck = Deck::new(DECK_SIZE);
        deck.shuffle_multi(&shuffle_types);
        let card_pos = deck.find_card_position(TARGET_CARD);
        let card_pos = deck.find_card_position(card_pos);
        let card_pos = deck.find_card_position(card_pos);
        let card_pos = deck.find_card_position(card_pos);
        let card_pos = deck.find_card_position(card_pos);
        let card_pos = deck.find_card_position(card_pos);
        let card_pos = deck.find_card_position(card_pos);
        card_pos as u64
    };

    let deck = BigDeck2::new(DECK_SIZE as u64);
    let shuffle = deck.compile(shuffle_types);
    let new_end_pos = deck.apply_shuffle(&shuffle, TARGET_CARD as u64);
    let new_end_pos = deck.apply_shuffle(&shuffle, new_end_pos);
    let new_end_pos = deck.apply_shuffle(&shuffle, new_end_pos);
    let new_end_pos = deck.apply_shuffle(&shuffle, new_end_pos);
    let new_end_pos = deck.apply_shuffle(&shuffle, new_end_pos);
    let new_end_pos = deck.apply_shuffle(&shuffle, new_end_pos);
    let new_end_pos = deck.apply_shuffle(&shuffle, new_end_pos);
    assert_eq!(new_end_pos, target_card_end_pos);

    let shuffle_stacked = deck.stack_shuffle(&shuffle, 7, 3);
    let new_end_pos = deck.apply_shuffle(&shuffle_stacked, TARGET_CARD as u64);
    assert_eq!(new_end_pos, target_card_end_pos);

    let rev_pos = deck.reverse_shuffle(&shuffle_stacked, new_end_pos);
    assert_eq!(rev_pos, TARGET_CARD as u64);

    println!("sanity check 3 passed");
}

fn main() {
    const DECK_SIZE: u64 = 119315717514047;
    const NUM_SHUFFLES: u64 = 101741582076661;
    const TARGET_CARD_POS: u64 = 2020;

    let input = aoc2019_utils::get_input("inputs/day22.txt");
    let shuffle_types = parse_input(&input);

    do_sanity_check(&shuffle_types);

    let deck = BigDeck2::new(DECK_SIZE);
    let shuffle = deck.compile(&shuffle_types);

    const SHUFFLE_STACK_CHUNK_SIZE: u64 = 10000000;
    let shuffle_stacked = deck.stack_shuffle(
        &shuffle,
        NUM_SHUFFLES,
        SHUFFLE_STACK_CHUNK_SIZE
    );
    let card_num = deck.reverse_shuffle(&shuffle_stacked, TARGET_CARD_POS);
    println!("card num: {}", card_num);
}
