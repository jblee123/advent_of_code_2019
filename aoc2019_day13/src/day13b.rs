pub mod day13_cpu;

use aoc2019_utils::*;

use day13_cpu::*;

const SCREEN_WIDTH: usize = 34;
const SCREEN_HEIGHT: usize = 34;

type Coord = point_2d::Point2d<i16>;
type Screen = Vec<Tile>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile { Empty, Wall, Block, Paddle, Ball }

impl Tile {
    fn from_num(num: i64) -> Self {
        match num {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("bad tile num"),
        }
    }
}

fn get_coord_idx(x: i64, y: i64) -> usize {
    (y as usize * SCREEN_HEIGHT) + x as usize
}

fn get_coord_from_idx(idx: usize) -> Coord {
    Coord {
        x: (idx % SCREEN_WIDTH) as i16,
        y: (idx / SCREEN_WIDTH) as i16,
    }
}

fn run_cycle(cpu: &mut Cpu, screen: &mut Screen) -> (bool, i64) {
    cpu.exec_prog();

    let mut score = 0;

    while cpu.has_output() {
        let x = cpu.pop_output().unwrap();
        let y = cpu.pop_output().unwrap();
        let data = cpu.pop_output().unwrap();

        if x >= 0 {
            screen[get_coord_idx(x, y)] = Tile::from_num(data);
        } else {
            score = data;
        }
    }

    let cont = cpu.get_state() != CpuState::Done;
    (cont, score)
}

fn get_num_blocks(screen: &Screen) -> usize {
    screen.iter().filter(|tile| **tile == Tile::Block).count()
}

fn get_ball_coord(screen: &Screen) -> Option<Coord> {
    screen.iter().enumerate()
        .filter(|(_, tile)| **tile == Tile::Ball)
        .next()
        .map(|(i, _)| {
            get_coord_from_idx(i)
        })
}

fn get_paddle_center(screen: &Screen) -> Coord {
    let paddle_coords = screen.iter().enumerate()
        .filter(|(_, tile)| **tile == Tile::Paddle)
        .map(|(i, _)| {
            get_coord_from_idx(i)
        })
        .collect::<Vec<Coord>>();

    let avg = paddle_coords.iter()
        .fold(Coord { x: 0, y: 0 }, |acc, coord| {
            acc + *coord
        })
        / paddle_coords.len() as i16;

    avg
}

fn get_next_input(ball: Coord, paddle_center: Coord) -> i64 {
    if ball.x < paddle_center.x {
        -1
    } else if ball.x > paddle_center.x {
        1
    } else {
        0
    }
}

fn main() {
    let input = get_input("inputs/day13.txt");
    let prog = {
        let mut prog = parse_prog(&input);
        prog[0] = 2;
        prog
    };
    let mut cpu = Cpu::new(&prog);
    cpu.set_print_output(false);

    let mut screen = vec![Tile::Empty; SCREEN_WIDTH * SCREEN_HEIGHT];

    println!("");

    loop {
        let (cont, score) = run_cycle(&mut cpu, &mut screen);
        let num_blocks = get_num_blocks(&screen);
        print!("\rblocks left: {}, score: {}                               ",
            num_blocks, score);

        if !cont {
            break;
        }

        let input = get_next_input(
            get_ball_coord(&screen).unwrap(),
            get_paddle_center(&screen),
        );

        cpu.add_input(input);
    }

    println!("");
}
