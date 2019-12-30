use std::cmp::Ordering;
use std::collections::HashMap;

use aoc2019_utils::*;

type Coord = point_2d::Point2d<i32>;

type Grid = Vec<Vec<char>>;

pub fn parse_input(input: &str) -> Grid {
    let height = input.lines().count();
    assert!(height > 0);
    let width = input.lines().next().unwrap().len();
    assert!(width > 0);

    let mut grid: Grid = (0..width).map(|_| {
        vec![' '; height]
    })
    .collect();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[x][y] = c;
        }
    }

    grid
}

fn gen_radial_search(start: Coord, width: usize, height: usize) -> Vec<Coord> {
    let mut coords = Vec::with_capacity(width * height);

    (0..width).for_each(|x| {
        (0..height).for_each(|y| {
            coords.push(Coord { x: x as i32, y: y as i32 });
        });
    });

    coords.sort_by(|a, b| {
        let dist_a = std::cmp::max(
            (a.x - start.x).abs(),
            (a.y - start.y).abs(),
        );
        let dist_b = std::cmp::max(
            (b.x - start.x).abs(),
            (b.y - start.y).abs(),
        );

        if dist_a < dist_b {
            return Ordering::Less;
        } else if dist_b < dist_a {
            return Ordering::Greater;
        }

        let get_angle_rad = |coord: Coord| {
            let dx = (coord.x - start.x) as f32;
            let dy = (start.y - coord.y) as f32;
            angle_to_0_2pi(dy.atan2(dx))
        };

        let angle_a = get_angle_rad(*a);
        let angle_b = get_angle_rad(*b);

        angle_a.partial_cmp(&angle_b).unwrap()
    });

    coords
}

fn num_visible_asteroids(start: Coord, grid: &Grid) -> u32 {
    let mut grid = grid.clone();

    let width = grid.len();
    let height = grid[0].len();
    let mut num_visible = 0;

    let coords = gen_radial_search(start, width, height);
    for coord in coords[1..].iter() {
        if grid[coord.x as usize][coord.y as usize] != '#' {
            continue;
        }

        num_visible += 1;
        let offset = Coord { x: coord.x - start.x, y: coord.y - start.y };
        let gcd = gcd(offset.x.abs() as u64, offset.y.abs() as u64);
        let offset = offset / gcd as i32;

        let mut new_coord = *coord;
        loop {
            new_coord += offset;

            if new_coord.x < 0
                || new_coord.y < 0
                || new_coord.x >= width as i32
                || new_coord.y >= height as i32
            {
                break;
            }

            if grid[new_coord.x as usize][new_coord.y as usize] == '#' {
                grid[new_coord.x as usize][new_coord.y as usize] = 'X';
            }
        }
    }

    num_visible
}

pub fn max_visible_asteroids_loc(grid: &Grid) -> Option<(u32, Coord)> {
    let width = grid.len();
    let height = grid[0].len();

    let mut max_visibles = None;
    let mut result = None;

    (0..width).for_each(|x| {
        (0..height).for_each(|y| {
            if grid[x][y] != '#' {
                return;
            }

            let coord = Coord { x: x as i32, y: y as i32 };
            let num_visible = num_visible_asteroids(coord, &grid);
            if !max_visibles.is_some()
                || (num_visible > max_visibles.unwrap())
            {
                max_visibles = Some(num_visible as u32);
                result = Some((num_visible as u32, coord));
            }
        });
    });

    result
}

fn collect_by_angles(grid: &Grid, center: Coord) -> Vec<Vec<Coord>> {
    let mut angle_buckets = HashMap::new();

    let width = grid.len();
    let height = grid[0].len();

    (0..width).for_each(|x| {
        (0..height).for_each(|y| {
            if grid[x][y] != '#' {
                return;
            }

            if x == center.x as usize && y == center.y as usize {
                return;
            }

            let dx = x as i32 - center.x;
            let dy = center.y - y as i32;

            let gcd = gcd(dx.abs() as u64, dy.abs() as u64);
            let dx = (dx / gcd as i32) as f32;
            let dy = (dy / gcd as i32) as f32;

            let ang = dy.atan2(dx);
            let ang = -(ang - std::f32::consts::FRAC_PI_2);
            let ang = angle_to_0_2pi(ang);

            if !angle_buckets.contains_key(&ang.to_bits()) {
                angle_buckets.insert(ang.to_bits(), vec![]);
            }

            angle_buckets.get_mut(&ang.to_bits()).unwrap()
                .push(Coord { x: x as i32, y: y as i32 });
        });
    });

    let mut angle_buckets = angle_buckets.drain().map(|(ang_bits, coords)| {
        (f32::from_bits(ang_bits), coords)
    }).collect::<Vec<(f32, Vec<Coord>)>>();

    angle_buckets.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut angle_buckets = angle_buckets.drain(..)
        .map(|(_, coords)| coords)
        .collect::<Vec<Vec<Coord>>>();

    for coords in &mut angle_buckets {
        coords.sort_by(|a, b| {
            let dist_a = std::cmp::max(
                (a.x - center.x).abs(),
                (a.y - center.y).abs(),
            );
            let dist_b = std::cmp::max(
                (b.x - center.x).abs(),
                (b.y - center.y).abs(),
            );
            dist_a.cmp(&dist_b)
        });
    }

    angle_buckets
}

pub fn get_nth_shot(grid: &Grid, center: Coord, shoot_num: u32) -> Option<Coord> {
    let mut angle_buckets = collect_by_angles(grid, center);

    let total_coords = angle_buckets.iter()
        .map(|bucket| bucket.len() as u32)
        .sum();

    if shoot_num > total_coords {
        return None;
    }

    let mut num_shot = 0;
    for bucket in &mut angle_buckets {
        if bucket.is_empty() {
            continue;
        }

        let coord = bucket.remove(0);
        num_shot += 1;
        if num_shot == shoot_num {
            return Some(coord);
        }
    }

    panic!("should never get here!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = concat!(
            ".#..#\n",
            ".....\n",
            "#####\n",
            "....#\n",
            "...##\n",
        );
        let target = vec![
            vec!['.', '.', '#', '.', '.'],
            vec!['#', '.', '#', '.', '.'],
            vec!['.', '.', '#', '.', '.'],
            vec!['.', '.', '#', '.', '#'],
            vec!['#', '.', '#', '#', '#'],
        ];
        let grid = parse_input(input);
        assert_eq!(grid, target);
    }

    #[test]
    fn test_gen_radial_search() {
        let result = gen_radial_search(Coord { x: 1, y: 1 }, 5, 5);
        let target = vec![
            Coord { x: 1, y: 1 },
            Coord { x: 2, y: 1 },
            Coord { x: 2, y: 0 },
            Coord { x: 1, y: 0 },
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 1 },
            Coord { x: 0, y: 2 },
            Coord { x: 1, y: 2 },
            Coord { x: 2, y: 2 },
            Coord { x: 3, y: 1 },
            Coord { x: 3, y: 0 },
            Coord { x: 0, y: 3 },
            Coord { x: 1, y: 3 },
            Coord { x: 2, y: 3 },
            Coord { x: 3, y: 3 },
            Coord { x: 3, y: 2 },
            Coord { x: 4, y: 1 },
            Coord { x: 4, y: 0 },
            Coord { x: 0, y: 4 },
            Coord { x: 1, y: 4 },
            Coord { x: 2, y: 4 },
            Coord { x: 3, y: 4 },
            Coord { x: 4, y: 4 },
            Coord { x: 4, y: 3 },
            Coord { x: 4, y: 2 },
        ];
        assert_eq!(result, target);
    }

    #[test]
    fn test_num_visible_asteroids() {
        let input = concat!(
            ".#..#\n",
            ".....\n",
            "#####\n",
            "....#\n",
            "...##\n",
        );
        let grid = parse_input(input);

        let result = num_visible_asteroids(Coord { x: 1, y: 0 }, &grid);
        assert_eq!(result, 7);

        let result = num_visible_asteroids(Coord { x: 4, y: 0 }, &grid);
        assert_eq!(result, 7);

        let result = num_visible_asteroids(Coord { x: 0, y: 2 }, &grid);
        assert_eq!(result, 6);

        let result = num_visible_asteroids(Coord { x: 2, y: 2 }, &grid);
        assert_eq!(result, 7);

        let result = num_visible_asteroids(Coord { x: 4, y: 2 }, &grid);
        assert_eq!(result, 5);

        let result = num_visible_asteroids(Coord { x: 3, y: 4 }, &grid);
        assert_eq!(result, 8);


        let input = concat!(
            ".....\n",
            ".....\n",
            "#.##.\n",
            ".....\n",
            ".....\n",
        );
        let grid = parse_input(input);
        let result = num_visible_asteroids(Coord { x: 0, y: 2 }, &grid);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_max_visible_asteroids_loc() {
        let input = concat!(
            ".#..#\n",
            ".....\n",
            "#####\n",
            "....#\n",
            "...##\n",
        );
        let grid = parse_input(input);
        let result = max_visible_asteroids_loc(&grid);
        assert_eq!(result, Some((8, Coord { x: 3, y: 4 })));

        let input = concat!(
            "......#.#.\n",
            "#..#.#....\n",
            "..#######.\n",
            ".#.#.###..\n",
            ".#..#.....\n",
            "..#....#.#\n",
            "#..#....#.\n",
            ".##.#..###\n",
            "##...#..#.\n",
            ".#....####\n",
        );
        let grid = parse_input(input);
        let result = max_visible_asteroids_loc(&grid);
        assert_eq!(result, Some((33, Coord { x: 5, y: 8 })));

        let input = concat!(
            "#.#...#.#.\n",
            ".###....#.\n",
            ".#....#...\n",
            "##.#.#.#.#\n",
            "....#.#.#.\n",
            ".##..###.#\n",
            "..#...##..\n",
            "..##....##\n",
            "......#...\n",
            ".####.###.\n",
        );
        let grid = parse_input(input);
        let result = max_visible_asteroids_loc(&grid);
        assert_eq!(result, Some((35, Coord { x: 1, y: 2 })));

        let input = concat!(
            ".#..#..###\n",
            "####.###.#\n",
            "....###.#.\n",
            "..###.##.#\n",
            "##.##.#.#.\n",
            "....###..#\n",
            "..#.#..#.#\n",
            "#..#.#.###\n",
            ".##...##.#\n",
            ".....#.#..\n",
        );
        let grid = parse_input(input);
        let result = max_visible_asteroids_loc(&grid);
        assert_eq!(result, Some((41, Coord { x: 6, y: 3 })));

        let input = concat!(
            ".#..##.###...#######\n",
            "##.############..##.\n",
            ".#.######.########.#\n",
            ".###.#######.####.#.\n",
            "#####.##.#.##.###.##\n",
            "..#####..#.#########\n",
            "####################\n",
            "#.####....###.#.#.##\n",
            "##.#################\n",
            "#####.##.###..####..\n",
            "..######..##.#######\n",
            "####.##.####...##..#\n",
            ".#####..#.######.###\n",
            "##...#.##########...\n",
            "#.##########.#######\n",
            ".####.#.###.###.#.##\n",
            "....##.##.###..#####\n",
            ".#.#.###########.###\n",
            "#.#.#.#####.####.###\n",
            "###.##.####.##.#..##\n",
        );
        let grid = parse_input(input);
        let result = max_visible_asteroids_loc(&grid);
        assert_eq!(result, Some((210, Coord { x: 11, y: 13 })));
    }

    // #[test]
    // fn test_collect_by_angles() {
    //     let input = concat!(
    //         ".#....#####...#..\n",
    //         "##...##.#####..##\n",
    //         "##...#...#.#####.\n",
    //         "..#.....X...###..\n",
    //         "..#.#.....#....##\n",
    //     );
    //     let grid = parse_input(input);
    //     let result = collect_by_angles(&grid, Coord { x: 8, y: 3 });
    //     println!("buckets:");
    //     for bucket in result {
    //         println!("  {:?}", bucket);
    //     }
    // }

    #[test]
    fn test_get_nth_shot() {
        let input = concat!(
            ".#..##.###...#######\n",
            "##.############..##.\n",
            ".#.######.########.#\n",
            ".###.#######.####.#.\n",
            "#####.##.#.##.###.##\n",
            "..#####..#.#########\n",
            "####################\n",
            "#.####....###.#.#.##\n",
            "##.#################\n",
            "#####.##.###..####..\n",
            "..######..##.#######\n",
            "####.##.####...##..#\n",
            ".#####..#.######.###\n",
            "##...#.##########...\n",
            "#.##########.#######\n",
            ".####.#.###.###.#.##\n",
            "....##.##.###..#####\n",
            ".#.#.###########.###\n",
            "#.#.#.#####.####.###\n",
            "###.##.####.##.#..##\n",
        );
        let grid = parse_input(input);
        let result = max_visible_asteroids_loc(&grid);
        assert_eq!(result, Some((210, Coord { x: 11, y: 13 })));
        let result = get_nth_shot(&grid, Coord { x: 11, y: 13 }, 200);
        assert_eq!(result, Some(Coord { x: 8, y: 2 }));
    }
}
