use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

fn main() {
    let program = get_contents("input");

    let mut robot = Robot::from_program(&program);

    robot.build_map();

    dbg!(bfs(&robot.map));
}

#[derive(Debug)]
struct Robot {
    computer: IntCodeComputer,
    map: HashMap<(i64, i64), i64>,
    position: (i64, i64),
    path: Vec<(i64, i64)>,
}

fn bfs(map: &HashMap<(i64, i64), i64>) -> i64 {
    let mut queue: VecDeque<((i64, i64), i64)> = VecDeque::new();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    let mut startpos = (0, 0);

    for (k, v) in map {
        if *v == 2 {
            startpos = *k;
        }
    }

    queue.push_back((startpos, 0));
    visited.insert(startpos);

    let mut maxdist = 0;

    while !(queue.len() == 0) {
        let (position, dist) = queue.pop_front().unwrap();
        if dist > maxdist {
            maxdist = dist;
        }
        let (px, py) = position;
        let search_positions = vec![(px, py + 1), (px, py - 1), (px - 1, py), (px + 1, py)];
        for sp in search_positions {
            let val = map[&sp];
            if val != 0 && !visited.contains(&sp) {
                queue.push_back((sp, dist + 1));
                visited.insert(sp);
            }
        }
    }

    maxdist
}

impl Robot {
    fn from_program(program: &str) -> Robot {
        Robot {
            computer: IntCodeComputer::from_program(program),
            map: {
                let mut m = HashMap::new();
                m.insert((0, 0), 1);
                m
            },
            position: (0, 0),
            path: vec![(0, 0)],
        }
    }

    fn build_map(&mut self) {
        loop {
            self.print_map();
            if self.search() == 3 {
                break;
            }
        }
    }

    fn print_map(&self) {
        let mut maxx = 0i64;
        let mut minx = 0i64;
        let mut maxy = 0i64;
        let mut miny = 0i64;

        for position in self.map.keys() {
            if position.0 < minx {
                minx = position.0;
            }
            if position.0 > maxx {
                maxx = position.0;
            }
            if position.1 < miny {
                miny = position.1;
            }
            if position.1 > maxy {
                maxy = position.1;
            }
        }

        let mut output_screen: String = String::new();
        output_screen.push_str(&format!(
            "minx: {}, maxx: {}, miny: {}, maxy: {}\n",
            minx, maxx, miny, maxy,
        ));

        for j in miny..maxy + 1 {
            for i in minx..maxx + 1 {
                if self.position == (i, j) {
                    output_screen.push('D');
                } else if self.map.contains_key(&(i, j)) {
                    output_screen.push(match self.map[&(i, j)] {
                        0 => '#',
                        1 => '.',
                        2 => 'o',
                        _ => panic!(),
                    });
                } else {
                    output_screen.push(' ');
                }
            }
            output_screen.push_str("\n");
        }
        println!("{}", output_screen);
    }

    fn search(&mut self) -> i64 {
        let (px, py) = self.position;
        let mut output = None;
        let search_positions = vec![(px, py + 1), (px, py - 1), (px - 1, py), (px + 1, py)];
        let commands = 1i64..5;
        for (search_position, command) in search_positions.iter().zip(commands) {
            if !self.map.contains_key(&search_position) {
                let mut o = self.computer.run(Some(command));
                if o.len() != 1 {
                    panic!();
                }
                let o = o.pop().unwrap();
                self.map.insert(*search_position, o);
                if o != 0 {
                    self.path.push(self.position);
                    self.position = *search_position
                }
                if o > 2 {
                    panic!();
                }
                output = Some(o);
                break;
            }
        }
        if !self.map.contains_key(&self.position) {
            panic!();
        }
        match output {
            Some(output) => output,
            None => {
                let p = self.position;
                let np = self.path.pop().unwrap();
                let input;
                if np.0 < p.0 {
                    input = 3;
                } else if np.0 > p.0 {
                    input = 4;
                } else if np.1 < p.1 {
                    input = 2;
                } else if np.1 > p.1 {
                    input = 1;
                } else {
                    return 3;
                }
                let output = self.computer.run(Some(input)).pop().unwrap();
                self.position = np;
                if output != 1 {
                    panic!()
                }
                output
            }
        }
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
