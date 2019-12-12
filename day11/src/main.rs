use ndarray::prelude::*;
use std::collections::{HashMap, HashSet};
use std::error;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input");
    let contents = contents.trim();

    let mut robot = Robot {
        position: (0, 0),
        direction: Direction::North,
        instructions: get_instructions(&contents),
        counter: 0,
        relative_base: 0,
    };

    let mut input = vec![1];
    let mut panel_colors: HashMap<(i64, i64), bool> = HashMap::new();
    let mut painted_panels: HashSet<(i64, i64)> = HashSet::new();
    let mut minx: i64 = 0;
    let mut miny: i64 = 0;
    let mut maxx: i64 = 0;
    let mut maxy: i64 = 0;

    panel_colors.insert((0, 0), true);

    loop {
        let (output, counter, relative_base, instructions) = run(
            &mut robot.instructions,
            &mut input,
            robot.counter,
            robot.relative_base,
        );
        robot.counter = counter;
        robot.relative_base = relative_base;
        robot.instructions = instructions.clone();
        if output.len() == 0 {
            if instructions[&robot.counter] != 99 {
                panic!();
            }
            break;
        }
        if output.len() != 2 {
            panic!();
        }
        let entry = panel_colors.entry(robot.position).or_insert(false);
        match output[0] {
            0 => *entry = false,
            1 => {
                *entry = true;
                painted_panels.insert(robot.position);
            }
            _ => panic!(),
        }
        match output[1] {
            0 => robot.direction = left(robot.direction),
            1 => robot.direction = right(robot.direction),
            _ => panic!(),
        }
        robot.position = match robot.direction {
            Direction::East => (robot.position.0 + 1, robot.position.1),
            Direction::West => (robot.position.0 - 1, robot.position.1),
            Direction::North => (robot.position.0, robot.position.1 + 1),
            Direction::South => (robot.position.0, robot.position.1 - 1),
        };
        if robot.position.0 < minx {
            minx = robot.position.0
        }
        if robot.position.0 > maxx {
            maxx = robot.position.0
        }
        if robot.position.1 < miny {
            miny = robot.position.1
        }
        if robot.position.1 > maxy {
            maxy = robot.position.1
        }
        input = match *panel_colors.entry(robot.position).or_insert(false) {
            true => vec![1],
            false => vec![0],
        }
    }

    let nx = (maxx - minx + 1) as usize;
    let ny = (maxy - miny + 1) as usize;

    let mut image: Array2<i64> = Array::zeros((nx, ny));

    for (p, c) in panel_colors {
        if c {
            image[((p.0 - minx) as usize, (p.1 - miny) as usize)] = 1;
        }
    }

    println!("{}", image);

    Ok(())
}

#[derive(Debug)]
struct Robot {
    position: (i64, i64),
    direction: Direction,
    instructions: HashMap<i64, i64>,
    relative_base: i64,
    counter: i64,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn left(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}

fn right(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn get_instructions(program: &str) -> HashMap<i64, i64> {
    let instructions: Vec<i64> = program.split(",").map(|x| x.parse().unwrap()).collect();
    let mut h_instructions: HashMap<i64, i64> = HashMap::new();
    for (i, ins) in instructions.iter().enumerate() {
        h_instructions.insert(i as i64, *ins);
    }
    h_instructions
}

fn run(
    instructions: &mut HashMap<i64, i64>,
    input: &mut Vec<i64>,
    mut counter: i64,
    mut relative_base: i64,
) -> (Vec<i64>, i64, i64, HashMap<i64, i64>) {
    let mut output: Vec<i64> = Vec::new();
    while instructions[&counter] != 99 {
        let instruction = instructions[&counter];
        let mut instruction = instruction
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        while instruction.len() < 5 {
            instruction.insert(0, 0);
        }
        let opcode = 10 * instruction[3] + instruction[4];
        let modes = &instruction[..3];
        let instruction_length;
        match opcode {
            1 => {
                let (args, addresses) = get_args(instructions, modes, counter, relative_base, 3);
                let entry = instructions.entry(addresses[2]).or_insert(0);
                *entry = args[0] + args[1];
                instruction_length = 4;
            }
            2 => {
                let (args, addresses) = get_args(instructions, modes, counter, relative_base, 3);
                let entry = instructions.entry(addresses[2]).or_insert(0);
                *entry = args[0] * args[1];
                instruction_length = 4;
            }
            3 => {
                let (_, addresses) = get_args(instructions, modes, counter, relative_base, 1);
                let entry = instructions.entry(addresses[0]).or_insert(0);
                *entry = input.pop().unwrap();
                instruction_length = 2;
            }
            4 => {
                let (args, _) = get_args(instructions, modes, counter, relative_base, 1);
                output.push(args[0]);
                if output.len() == 2 {
                    counter += 2;
                    return (output, counter, relative_base, instructions.clone());
                }
                instruction_length = 2;
            }
            5 => {
                let (args, _) = get_args(instructions, modes, counter, relative_base, 2);
                if args[0] != 0 {
                    counter = args[1];
                    instruction_length = 0;
                } else {
                    instruction_length = 3;
                }
            }
            6 => {
                let (args, _) = get_args(instructions, modes, counter, relative_base, 2);
                if args[0] == 0 {
                    counter = args[1];
                    instruction_length = 0;
                } else {
                    instruction_length = 3;
                }
            }
            7 => {
                let (args, addresses) = get_args(instructions, modes, counter, relative_base, 3);
                let res = match args[0] < args[1] {
                    true => 1,
                    false => 0,
                };
                let entry = instructions.entry(addresses[2]).or_insert(0);
                *entry = res;
                instruction_length = 4;
            }
            8 => {
                let (args, addresses) = get_args(instructions, modes, counter, relative_base, 3);
                let res = match args[0] == args[1] {
                    true => 1,
                    false => 0,
                };
                let entry = instructions.entry(addresses[2]).or_insert(0);
                *entry = res;
                instruction_length = 4;
            }
            9 => {
                let (args, _) = get_args(instructions, modes, counter, relative_base, 1);
                relative_base += args[0];
                instruction_length = 2;
            }
            _ => panic!(),
        }
        counter += instruction_length;
    }
    (output, counter, relative_base, instructions.clone())
}

fn get_args(
    instructions: &mut HashMap<i64, i64>,
    modes: &[u32],
    counter: i64,
    relative_base: i64,
    num_args: usize,
) -> (Vec<i64>, Vec<i64>) {
    let mut args: Vec<i64> = Vec::new();
    let mut addresses: Vec<i64> = Vec::new();
    for arg_count in 0..num_args {
        let mut address = instructions[&(counter + arg_count as i64 + 1)];
        let arg;
        let mode = match arg_count {
            0 => modes[2],
            1 => modes[1],
            2 => {
                if modes[0] == 1 {
                    panic!()
                };
                modes[0]
            }
            _ => {
                panic!();
            }
        };
        if mode == 0 {
            arg = instructions.entry(address).or_insert(0);
        } else if mode == 1 {
            arg = &mut address;
        } else if mode == 2 {
            address = address + relative_base;
            arg = instructions.entry(address).or_insert(0);
        } else {
            panic!();
        }
        args.push(*arg);
        addresses.push(address);
    }
    (args, addresses)
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
    // use super::*;

    #[test]
    fn test() {}
}
