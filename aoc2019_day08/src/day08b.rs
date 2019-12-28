pub mod day08_utils;

use aoc2019_utils;

use day08_utils::*;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let input = aoc2019_utils::get_input("inputs/day08.txt");
    let img = decode_image(&input, 25, 6);
    let input = input.as_bytes();

    let mut canvas = [[b' '; HEIGHT]; WIDTH];

    let pix_in_image = WIDTH * HEIGHT;
    (0..img.layers.len()).rev().for_each(|layer_num| {
        let mut idx = layer_num * pix_in_image;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                canvas[x][y] = match input[idx] {
                    b'0' => b' ',
                    b'1' => b'#',
                    b'2' => canvas[x][y],
                    _ => panic!("unexpected byte: {}", input[idx] as char),
                };

                idx += 1;
            }
        }
    });

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", canvas[x][y] as char);
        }
        println!("");
    }
}
