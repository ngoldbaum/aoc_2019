use std::error;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input");

    let min_distance = get_min_distance(&contents);

    dbg!(min_distance);

    Ok(())
}

fn get_min_distance(contents: &str) -> usize {
    let instructions = interpret_contents(contents);

    let point_sets = walk_instructions(instructions);

    let mut intersections: Vec<usize> = Vec::new();

    for p0 in &point_sets.0 {
        for p1 in &point_sets.1 {
            if (p0.x == p1.x) && (p0.y == p1.y) {
                intersections.push(p0.step_count + p1.step_count);
            }
        }
    }

    *intersections.iter().min().unwrap()
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    step_count: usize,
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Instruction {
    direction: Direction,
    step_size: usize,
}

fn interpret_contents(contents: &str) -> (Vec<Instruction>, Vec<Instruction>) {
    let mut result: Vec<Vec<Instruction>> = Vec::new();
    for line in contents.lines() {
        result.push(
            line.split(",")
                .map(|i| Instruction {
                    direction: {
                        match i.chars().next().unwrap() {
                            'R' => Direction::Right,
                            'L' => Direction::Left,
                            'U' => Direction::Up,
                            'D' => Direction::Down,
                            _ => panic!(),
                        }
                    },
                    step_size: { i[1..].parse::<usize>().unwrap() },
                })
                .collect(),
        );
    }

    (result[0].clone(), result[1].clone())
}

fn walk_instructions(
    instructions: (Vec<Instruction>, Vec<Instruction>),
) -> (Vec<Point>, Vec<Point>) {
    (
        walk_instruction_vec(instructions.0),
        walk_instruction_vec(instructions.1),
    )
}

fn walk_instruction_vec(instruction_vec: Vec<Instruction>) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();

    let mut x = 0;
    let mut y = 0;
    let mut step_count = 0;

    for instruction in instruction_vec {
        for _ in 0..instruction.step_size {
            match instruction.direction {
                Direction::Up => y += 1,
                Direction::Down => y -= 1,
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
            }
            step_count += 1;
            result.push(Point { x, y, step_count });
        }
    }

    result.retain(|p| !(p.x == 0 && p.y == 0));

    result
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
        assert!(get_min_distance("R8,U5,L5,D3\nU7,R6,D4,L4") == 30);
        assert!(
            get_min_distance("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")
                == 610
        );
        assert!(
            get_min_distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ) == 410
        );
    }
}
