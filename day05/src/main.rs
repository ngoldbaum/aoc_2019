use std::error;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input");
    let contents = contents.trim();

    dbg!(run(&contents, 1));

    dbg!(run(&contents, 5));

    Ok(())
}

fn run(program: &str, input: i64) -> i64 {
    let mut instructions: Vec<i64> = program.split(",").map(|x| x.parse().unwrap()).collect();
    let mut counter = 0;
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
                instructions[address] = input;
                instruction_length = 2;
            }
            4 => {
                let mut output = instructions[counter + 1];
                if modes[2] == 0 {
                    output = instructions[output as usize];
                }
                if output != 0 {
                    return output;
                }
                instruction_length = 2;
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
    0
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
        assert!(run("3,9,8,9,10,9,4,9,99,-1,8", 8) == 1);
        assert!(run("3,9,8,9,10,9,4,9,99,-1,8", 7) == 0);
        assert!(run("3,9,8,9,10,9,4,9,99,-1,8", 9) == 0);

        assert!(run("3,9,7,9,10,9,4,9,99,-1,8", 8) == 0);
        assert!(run("3,9,7,9,10,9,4,9,99,-1,8", 7) == 1);
        assert!(run("3,9,7,9,10,9,4,9,99,-1,8", 3) == 1);
        assert!(run("3,9,7,9,10,9,4,9,99,-1,8", 9) == 0);

        assert!(run("3,3,1108,-1,8,3,4,3,99", 8) == 1);
        assert!(run("3,3,1108,-1,8,3,4,3,99", 7) == 0);
        assert!(run("3,3,1108,-1,8,3,4,3,99", 9) == 0);

        assert!(run("3,3,1107,-1,8,3,4,3,99", 8) == 0);
        assert!(run("3,3,1107,-1,8,3,4,3,99", 7) == 1);
        assert!(run("3,3,1107,-1,8,3,4,3,99", 3) == 1);
        assert!(run("3,3,1107,-1,8,3,4,3,99", 9) == 0);

        assert!(run("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0) == 0);
        assert!(run("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 12) == 1);
        assert!(run("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", -1) == 1);

        assert!(run("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0) == 0);
        assert!(run("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 12) == 1);
        assert!(run("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", -1) == 1);

        let long_program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

        assert!(run(long_program, 6) == 999);
        assert!(run(long_program, 7) == 999);
        assert!(run(long_program, 8) == 1000);
        assert!(run(long_program, 9) == 1001);
        assert!(run(long_program, 10) == 1001);
    }
}
