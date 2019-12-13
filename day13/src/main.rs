use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let program = get_contents("input");

    let mut game = ArcadeCabinet::from_program(&program);

    let mut input: Vec<i64> = vec![];

    loop {
        let output_screen = game.run(input);
        print!("{}[2J", 27 as char);
        println!("{}", output_screen.trim());
        let paddle_position = game.find(3);
        let ball_position = game.find(4);
        if paddle_position < ball_position {
            input = vec![1];
        } else if paddle_position > ball_position {
            input = vec![-1];
        } else {
            input = vec![0];
        }
        sleep(Duration::from_millis(17));
    }
}

#[derive(Debug)]
struct ArcadeCabinet {
    computer: IntCodeComputer,
    screen: HashMap<(i64, i64), i64>,
    score: i64,
}

impl ArcadeCabinet {
    fn from_program(program: &str) -> ArcadeCabinet {
        ArcadeCabinet {
            computer: IntCodeComputer::from_program(program),
            screen: HashMap::new(),
            score: 0,
        }
    }

    fn run(&mut self, init_input: Vec<i64>) -> String {
        let output = self.computer.run(&init_input);

        for tile_data in output.chunks(3) {
            if (tile_data[0], tile_data[1]) == (-1, 0) {
                self.score = tile_data[2];
                continue;
            }
            self.screen
                .insert((tile_data[0], tile_data[1]), tile_data[2]);
        }

        let mut maxx = 0i64;
        let mut minx = 0i64;
        let mut maxy = 0i64;
        let mut miny = 0i64;

        for positions in self.screen.keys() {
            if positions.0 < minx {
                minx = positions.0;
            }
            if positions.0 > maxx {
                maxx = positions.0;
            }
            if positions.0 < miny {
                miny = positions.0;
            }
            if positions.0 > maxy {
                maxy = positions.0;
            }
        }

        let mut output_screen: String = String::new();

        output_screen.push_str(&format!("Score = {}\n", self.score));

        for j in miny..maxy + 1 {
            for i in minx..maxx + 1 {
                if self.screen.contains_key(&(i, j)) {
                    output_screen.push(match self.screen[&(i, j)] {
                        0 => ' ',
                        1 => '#',
                        2 => '▩',
                        3 => '_',
                        4 => 'o',
                        _ => panic!(),
                    });
                } else {
                    output_screen.push(' ')
                }
            }
            output_screen.push_str("\n");
        }
        output_screen
    }

    fn find(&self, index: i64) -> i64 {
        for (k, v) in &self.screen {
            if *v == index {
                return k.0;
            }
        }
        panic!();
    }
}

#[derive(Debug)]
struct IntCodeComputer {
    instructions: HashMap<i64, i64>,
    counter: i64,
    relative_base: i64,
}

impl IntCodeComputer {
    fn run(&mut self, input: &Vec<i64>) -> Vec<i64> {
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
                    match input.pop() {
                        Some(value) => *entry = value,
                        None => return output,
                    }
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
