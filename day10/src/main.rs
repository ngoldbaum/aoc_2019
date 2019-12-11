use std::fs::File;
use std::io::Read;

use indoc::indoc;
use ordered_float::OrderedFloat;
use std::collections::HashSet;

fn main() {
    let map = get_contents("input");

    dbg!(get_maxnum(&map));
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

fn get_asteroid_positions(map: &str) -> Vec<(i64, i64)> {
    let mut asteroid_positions: Vec<(i64, i64)> = Vec::new();
    for (j, line) in map.lines().enumerate() {
        for (i, chr) in line.chars().enumerate() {
            if chr == '#' {
                asteroid_positions.push((i as i64, j as i64));
            }
        }
    }
    asteroid_positions
}

fn get_num_can_see(position: (i64, i64), mut asteroid_positions: Vec<(i64, i64)>) -> usize {
    asteroid_positions.sort_by_key(|p| {
        OrderedFloat((((p.0 - position.0).pow(2) + (p.1 - position.1).pow(2)) as f64).sqrt())
    });
    let mut seen_angles: HashSet<OrderedFloat<f64>> = HashSet::new();
    let mut num_can_see = 0;
    for p in asteroid_positions {
        let angle = OrderedFloat(((position.1 - p.1) as f64).atan2((position.0 - p.0) as f64));
        if !seen_angles.contains(&angle) {
            num_can_see += 1;
            seen_angles.insert(angle);
        }
    }
    num_can_see
}

fn get_maxnum(map: &str) -> (usize, (i64, i64)) {
    let positions = get_asteroid_positions(map);
    let mut maxp: (i64, i64) = (-1, -1);
    let mut maxnum = 0;
    for (i, p) in positions.iter().enumerate() {
        let mut pos_wout_p = positions.clone();
        pos_wout_p.remove(i);
        let num = get_num_can_see(*p, pos_wout_p);
        if num > maxnum {
            maxnum = num;
            maxp = *p;
        }
    }
    (maxnum, maxp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let map = indoc!(
            ".#..#
             .....
             #####
             ....#
             ...##"
        );

        assert!(get_maxnum(map) == (8, (3, 4)));

        let map = indoc!(
            "......#.#.
             #..#.#....
             ..#######.
             .#.#.###..
             .#..#.....
             ..#....#.#
             #..#....#.
             .##.#..###
             ##...#..#.
             .#....####"
        );

        assert!(get_maxnum(map) == (33, (5, 8)));

        let map = indoc!(
            "#.#...#.#.
             .###....#.
             .#....#...
             ##.#.#.#.#
             ....#.#.#.
             .##..###.#
             ..#...##..
             ..##....##
             ......#...
             .####.###."
        );

        assert!(get_maxnum(map) == (35, (1, 2)));

        let map = indoc!(
            ".#..#..###
             ####.###.#
             ....###.#.
             ..###.##.#
             ##.##.#.#.
             ....###..#
             ..#.#..#.#
             #..#.#.###
             .##...##.#
             .....#.#.."
        );

        assert!(get_maxnum(map) == (41, (6, 3)));

        let map = indoc!(
            ".#..##.###...#######
             ##.############..##.
             .#.######.########.#
             .###.#######.####.#.
             #####.##.#.##.###.##
             ..#####..#.#########
             ####################
             #.####....###.#.#.##
             ##.#################
             #####.##.###..####..
             ..######..##.#######
             ####.##.####...##..#
             .#####..#.######.###
             ##...#.##########...
             #.##########.#######
             .####.#.###.###.#.##
             ....##.##.###..#####
             .#.#.###########.###
             #.#.#.#####.####.###
             ###.##.####.##.#..##"
        );

        assert!(get_maxnum(map) == (210, (11, 13)));
    }
}
