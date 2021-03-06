use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut computer = IntCodeComputer::from_program(&get_contents("input"));
    let output: Vec<u8> = computer.run(None).iter().map(|x| *x as u8).collect();
    let map_str = std::str::from_utf8(&output).unwrap().trim();
    println!("{}", &map_str);
    let map: Vec<Vec<char>> = map_str.lines().map(|x| x.chars().collect()).collect();
    let mut parsum = 0;
    for j in 1..map.len() - 1 {
        for i in 1..map[j].len() - 1 {
            if (
                map[j][i],
                map[j - 1][i],
                map[j + 1][i],
                map[j][i - 1],
                map[j][i + 1],
            ) == ('#', '#', '#', '#', '#')
            {
                parsum += i * j;
            }
        }
    }
    dbg!(parsum);
    let mut pos: Option<(i64, i64)> = None;
    let mut direction: Option<Direction> = None;
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            if ['<', '^', '>', 'v'].contains(&map[j][i]) {
                pos = Some((i as i64, j as i64));
                direction = Some(get_direction(map[j][i]));
            }
        }
    }
    let pos = pos.unwrap();
    let direction = direction.unwrap();
    let mut robot = Robot::from(pos, direction, map);
    while robot.position != pos {
        robot.walk()
    }
}

fn get_direction(c: char) -> Direction {
    match c {
        '>' => Direction::East,
        '^' => Direction::North,
        '<' => Direction::West,
        'v' => Direction::South,
        _ => panic!(),
    }
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

#[derive(Debug)]
struct Robot {
    position: (i64, i64),
    path: Vec<(i64, i64)>,
    direction: Direction,
    map: Vec<Vec<char>>,
}

impl Robot {
    fn from(pos: (i64, i64), direction: Direction, map: Vec<Vec<char>>) -> Robot {
        let path = vec![pos];
        Robot {
            position: pos,
            path: path,
            direction: direction,
            map: map,
        }
    }

    fn walk(&mut self) {
        let px = self.position.0 as usize;
        let py = self.position.1 as usize;
        let mut neighborhood: [[char; 3]; 3] = [[' '; 3]; 3];
        for (i, pi) in (px - 1..px + 2).enumerate() {
            for (j, pj) in (py - 1..py + 2).enumerate() {
                neighborhood[j][i] = self.map[pj][pi];
            }
        }
        let neighborhood: Vec<String> = neighborhood
            .iter()
            .map(|x| x.iter().collect::<String>())
            .collect();
        let neighborhood: String = neighborhood.join("\n");
        println!("{}", neighborhood);
    }
}

#[derive(Debug)]
struct IntCodeComputer {
    instructions: HashMap<i64, i64>,
    counter: i64,
    relative_base: i64,
}

impl IntCodeComputer {
    fn run(&mut self, input: Option<i64>) -> Vec<i64> {
        let mut input = input.clone();
        let mut output: Vec<i64> = Vec::new();
        while self.instructions[&self.counter] != 99 {
            let instruction = self.instructions[&self.counter];
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
                    let (args, addresses) = self.get_args(modes, 3);
                    let entry = self.instructions.entry(addresses[2]).or_insert(0);
                    *entry = args[0] + args[1];
                    instruction_length = 4;
                }
                2 => {
                    let (args, addresses) = self.get_args(modes, 3);
                    let entry = self.instructions.entry(addresses[2]).or_insert(0);
                    *entry = args[0] * args[1];
                    instruction_length = 4;
                }
                3 => {
                    let (_, addresses) = self.get_args(modes, 1);
                    let entry = self.instructions.entry(addresses[0]).or_insert(0);
                    match input {
                        Some(value) => *entry = value,
                        None => return output,
                    }
                    input = None;
                    instruction_length = 2;
                }
                4 => {
                    let (args, _) = self.get_args(modes, 1);
                    output.push(args[0]);
                    instruction_length = 2;
                }
                5 => {
                    let (args, _) = self.get_args(modes, 2);
                    if args[0] != 0 {
                        self.counter = args[1];
                        instruction_length = 0;
                    } else {
                        instruction_length = 3;
                    }
                }
                6 => {
                    let (args, _) = self.get_args(modes, 2);
                    if args[0] == 0 {
                        self.counter = args[1];
                        instruction_length = 0;
                    } else {
                        instruction_length = 3;
                    }
                }
                7 => {
                    let (args, addresses) = self.get_args(modes, 3);
                    let res = match args[0] < args[1] {
                        true => 1,
                        false => 0,
                    };
                    let entry = self.instructions.entry(addresses[2]).or_insert(0);
                    *entry = res;
                    instruction_length = 4;
                }
                8 => {
                    let (args, addresses) = self.get_args(modes, 3);
                    let res = match args[0] == args[1] {
                        true => 1,
                        false => 0,
                    };
                    let entry = self.instructions.entry(addresses[2]).or_insert(0);
                    *entry = res;
                    instruction_length = 4;
                }
                9 => {
                    let (args, _) = self.get_args(modes, 1);
                    self.relative_base += args[0];
                    instruction_length = 2;
                }
                _ => panic!(),
            }
            self.counter += instruction_length;
        }
        output
    }

    fn get_args(&mut self, modes: &[u32], num_args: usize) -> (Vec<i64>, Vec<i64>) {
        let mut args: Vec<i64> = Vec::new();
        let mut addresses: Vec<i64> = Vec::new();
        for arg_count in 0..num_args {
            let mut address = self.instructions[&(self.counter + arg_count as i64 + 1)];
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
                arg = self.instructions.entry(address).or_insert(0);
            } else if mode == 1 {
                arg = &mut address;
            } else if mode == 2 {
                address = address + self.relative_base;
                arg = self.instructions.entry(address).or_insert(0);
            } else {
                panic!();
            }
            args.push(*arg);
            addresses.push(address);
        }
        (args, addresses)
    }

    fn from_program(program: &str) -> IntCodeComputer {
        let instructions: Vec<i64> = program.split(",").map(|x| x.parse().unwrap()).collect();
        let mut h_instructions: HashMap<i64, i64> = HashMap::new();
        for (i, ins) in instructions.iter().enumerate() {
            h_instructions.insert(i as i64, *ins);
        }
        IntCodeComputer {
            instructions: h_instructions,
            counter: 0,
            relative_base: 0,
        }
    }
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents.trim().to_string()
}
