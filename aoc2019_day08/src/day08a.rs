pub mod day08_utils;

use aoc2019_utils;

use day08_utils::*;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day08.txt");
    let img = decode_image(&input, 25, 6);
    let min_zero_cnt = img.layers.iter()
        .map(|layer| layer.digit_counts[0])
        .min()
        .unwrap();
    let target_layer = img.layers.iter()
        .find(|layer| {
            layer.digit_counts[0] == min_zero_cnt
        })
        .unwrap();

    let val = target_layer.digit_counts[1] * target_layer.digit_counts[2];
    println!("val: {}", val);
}
