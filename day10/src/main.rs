use std::fs::File;
use std::io::Read;

use indoc::indoc;
use ordered_float::OrderedFloat;
use std::collections::HashMap;

fn main() {
    let map = get_contents("input");

    let (_, maxp) = get_maxnum(&map);

    let mut positions = get_asteroid_positions(&map);

    positions.remove(positions.iter().position(|x| *x == maxp).unwrap());

    let mut last_angle = std::f64::consts::PI / 2. - 1e-10;
    let mut num_vaporized = 0;

    while positions.len() != 1 {
        last_angle = vaporize(maxp, &mut positions, last_angle, &mut num_vaporized);
    }
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

fn get_num_can_see(
    position: (i64, i64),
    mut asteroid_positions: Vec<(i64, i64)>,
) -> (usize, HashMap<OrderedFloat<f64>, (i64, i64)>) {
    asteroid_positions.sort_by_key(|p| {
        OrderedFloat((((p.0 - position.0).pow(2) + (p.1 - position.1).pow(2)) as f64).sqrt())
    });
    let mut seen_angles: HashMap<OrderedFloat<f64>, (i64, i64)> = HashMap::new();
    let mut num_can_see = 0;
    for p in asteroid_positions {
        let angle = OrderedFloat(((position.1 - p.1) as f64).atan2((position.0 - p.0) as f64));
        if !seen_angles.contains_key(&angle) {
            num_can_see += 1;
            seen_angles.insert(angle, p);
        }
    }
    (num_can_see, seen_angles)
}

fn vaporize(
    position: (i64, i64),
    asteroid_positions: &mut Vec<(i64, i64)>,
    last_angle: f64,
    num_vaporized: &mut usize,
) -> f64 {
    asteroid_positions.sort_by_key(|p| {
        OrderedFloat((((p.0 - position.0).pow(2) + (p.1 - position.1).pow(2)) as f64).sqrt())
    });
    let mut seen_angles: HashMap<OrderedFloat<f64>, (i64, i64)> = HashMap::new();
    for p in asteroid_positions.iter() {
        let angle = OrderedFloat(((position.1 - p.1) as f64).atan2((position.0 - p.0) as f64));
        if !seen_angles.contains_key(&angle) {
            seen_angles.insert(angle, *p);
        }
    }
    let mut angles: Vec<OrderedFloat<f64>> = seen_angles.keys().map(|x| *x).collect();
    angles.sort();
    let mut ret_angle = -4.0;
    for angle in angles {
        if *angle > last_angle {
            let p = seen_angles[&angle];
            asteroid_positions.remove(asteroid_positions.iter().position(|x| *x == p).unwrap());
            *num_vaporized += 1;
            println!("Vaporizing asteroid {} at position {:?}", *num_vaporized, p);
            ret_angle = *angle;
        }
    }
    ret_angle
}

fn get_maxnum(map: &str) -> (usize, (i64, i64)) {
    let positions = get_asteroid_positions(map);
    let mut maxp: (i64, i64) = (-1, -1);
    let mut maxnum = 0;
    for (i, p) in positions.iter().enumerate() {
        let mut pos_wout_p = positions.clone();
        pos_wout_p.remove(i);
        let (num, _) = get_num_can_see(*p, pos_wout_p);
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
