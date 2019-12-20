use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let contents = get_contents("input");
    let mut beam_image: HashMap<(i64, i64), char> = HashMap::new();
    let mut row = 1330;
    while dbg!(get_nrow(row, &mut beam_image, &contents)) != 1 {
        row += 1;
        dbg!(row);
    }
    let mut dist = 1000000000.;
    for row in 1330..1335 {
        for i in 0..1500 {
            if in_beam((i as i64, row as i64), &mut beam_image, &contents)
                && in_beam(((i + 99) as i64, row as i64), &mut beam_image, &contents)
                && in_beam((i as i64, (row + 99) as i64), &mut beam_image, &contents)
            {
                let score = i * 10000 + row;
                if ((i as f64).powi(2) + (row as f64).powi(2)).sqrt() < dist {
                    dist = ((i as f64).powi(2) + (row as f64).powi(2)).sqrt();
                    dbg!(((i, row), score));
                }
            }
        }
    }
    dbg!(dist);
    //println!(
    //    "{}",
    //    beam_image
    //        .iter()
    //        .map(|x| x.iter().collect::<String>())
    //        .collect::<Vec<String>>()
    //        .join("\n")
    //);
}

fn get_nrow(rownum: usize, beam_image: &mut HashMap<(i64, i64), char>, contents: &str) -> usize {
    let mut nrow = 0;
    for i in 0..1500 {
        if in_beam((i as i64, rownum as i64), beam_image, contents) {
            if in_beam(((i + 99) as i64, rownum as i64), beam_image, contents)
                && in_beam((i as i64, (rownum + 99) as i64), beam_image, contents)
            {
                nrow += 1
            }
        }
    }
    nrow
}

fn in_beam(coords: (i64, i64), beam_image: &mut HashMap<(i64, i64), char>, contents: &str) -> bool {
    if beam_image.contains_key(&coords) {
        return match beam_image[&coords] {
            '#' => true,
            '.' => false,
            _ => panic!(),
        };
    }
    let mut computer = IntCodeComputer::from_program(contents);
    let output: Vec<i64> = computer.run(Some(coords.0 as i64));
    assert!(output.len() == 0);
    let output: Vec<i64> = computer.run(Some(coords.1 as i64));
    assert!(output.len() == 1);
    if output[0] == 1 {
        beam_image.insert(coords, '#');
        return true;
    } else {
        beam_image.insert(coords, '.');
        return false;
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
