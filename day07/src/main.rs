use std::error;
use std::fs::File;
use std::io::Read;

use itertools::Itertools;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input");
    let contents = contents.trim();

    dbg!(run_amplifier_combinations(&contents));

    Ok(())
}

fn run_amplifier_combinations(program: &str) -> i64 {
    let mut max_output = 0;

    for p in (5..10).permutations(5) {
        let output = run_amplifiers(program, [p[0], p[1], p[2], p[3], p[4]]);
        if output > max_output {
            max_output = output;
        }
    }

    max_output
}

fn run_amplifiers(program: &str, settings: [i64; 5]) -> i64 {
    let mut counters = vec![0; 5];
    let instructions: Vec<i64> = program.split(",").map(|x| x.parse().unwrap()).collect();
    let mut programs = vec![instructions; 5];
    let mut inputs: Vec<Vec<i64>> = Vec::new();
    for setting in &settings {
        inputs.push(vec![*setting]);
    }
    inputs[0].insert(0, 0);
    let mut output = 0;
    for i in 0..5 {
        let (some_output, counter, instructions) =
            run(&mut programs[i], &mut inputs[i], counters[i], output);
        output = some_output;
        programs[i] = instructions;
        inputs[(i + 1) % 5].insert(0, output);
        counters[i] = counter;
    }

    loop {
        for i in 0..5 {
            let (some_output, counter, instructions) =
                run(&mut programs[i], &mut inputs[i], counters[i], output);
            output = some_output;
            programs[i] = instructions;
            inputs[(i + 1) % 5].insert(0, output);
            counters[i] = counter;
        }
        if programs[4][counters[4]] == 99 {
            break;
        }
    }

    output
}

fn run(
    instructions: &mut Vec<i64>,
    input: &mut Vec<i64>,
    mut counter: usize,
    mut output: i64,
) -> (i64, usize, Vec<i64>) {
    while instructions[counter] != 99 {
        let instruction = instructions[counter];
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
            1 | 2 => {
                let mut arg0 = instructions[counter + 1];
                if modes[2] == 0 {
                    arg0 = instructions[arg0 as usize];
                }
                let mut arg1 = instructions[counter + 2];
                if modes[1] == 0 {
                    arg1 = instructions[arg1 as usize];
                }
                if modes[0] == 1 {
                    dbg!(&instruction);
                    dbg!(&instructions[counter]);
                    panic!();
                }
                let res;
                if opcode == 1 {
                    res = arg0 + arg1;
                } else if opcode == 2 {
                    res = arg0 * arg1;
                } else {
                    panic!();
                }
                let address = instructions[counter + 3];
                instructions[address as usize] = res;
                instruction_length = 4;
            }
            3 => {
                let address = instructions[counter + 1] as usize;
                instructions[address] = input.pop().unwrap();
                instruction_length = 2;
            }
            4 => {
                output = instructions[counter + 1];
                if modes[2] == 0 {
                    output = instructions[output as usize];
                }
                counter += 2;
                return (output, counter, instructions.clone());
            }
            5 => {
                let mut arg0 = instructions[counter + 1];
                if modes[2] == 0 {
                    arg0 = instructions[arg0 as usize]
                }
                let mut arg1 = instructions[counter + 2];
                if modes[1] == 0 {
                    arg1 = instructions[arg1 as usize];
                }
                if arg0 != 0 {
                    counter = arg1 as usize;
                    instruction_length = 0;
                } else {
                    instruction_length = 3;
                }
            }
            6 => {
                let mut arg0 = instructions[counter + 1];
                if modes[2] == 0 {
                    arg0 = instructions[arg0 as usize]
                }
                let mut arg1 = instructions[counter + 2];
                if modes[1] == 0 {
                    arg1 = instructions[arg1 as usize];
                }
                if arg0 == 0 {
                    counter = arg1 as usize;
                    instruction_length = 0;
                } else {
                    instruction_length = 3;
                }
            }
            7 => {
                let mut arg0 = instructions[counter + 1];
                if modes[2] == 0 {
                    arg0 = instructions[arg0 as usize]
                }
                let mut arg1 = instructions[counter + 2];
                if modes[1] == 0 {
                    arg1 = instructions[arg1 as usize];
                }
                let address = instructions[counter + 3] as usize;
                if arg0 < arg1 {
                    instructions[address] = 1;
                } else {
                    instructions[address] = 0;
                }
                instruction_length = 4;
            }
            8 => {
                let mut arg0 = instructions[counter + 1];
                if modes[2] == 0 {
                    arg0 = instructions[arg0 as usize]
                }
                let mut arg1 = instructions[counter + 2];
                if modes[1] == 0 {
                    arg1 = instructions[arg1 as usize];
                }
                let address = instructions[counter + 3] as usize;
                if arg0 == arg1 {
                    instructions[address] = 1;
                } else {
                    instructions[address] = 0;
                }
                instruction_length = 4;
            }
            _ => panic!(),
        }
        counter += instruction_length;
    }
    (output, counter, instructions.clone())
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
    fn test() {
        assert!(run_amplifier_combinations("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5") == 139629729);
        assert!(run_amplifier_combinations("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10") == 18216);
    }
}
