use std::error;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input");

    let result = run(&replace(&contents, 12, 2));

    dbg!(get_output(&result));

    let mut done = false;

    for noun in 0..99 {
        for verb in 0..99 {
            if get_output(&run(&replace(&contents, noun, verb))) == 19690720 {
                dbg!(100 * noun + verb);
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }

    Ok(())
}

fn get_output(output: &str) -> usize {
    output.split(",").collect::<Vec<&str>>()[0].parse().unwrap()
}

fn replace(program: &str, noun: usize, verb: usize) -> String {
    let mut instructions: Vec<usize> = program.split(",").map(|x| x.parse().unwrap()).collect();
    instructions[1] = noun;
    instructions[2] = verb;
    instructions
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn run(program: &str) -> String {
    let mut instructions: Vec<usize> = program.split(",").map(|x| x.parse().unwrap()).collect();
    let mut counter = 0;
    while instructions[counter] != 99 {
        let instruction = instructions[counter];
        let res;
        if instruction == 1 {
            res = instructions[instructions[counter + 1]] + instructions[instructions[counter + 2]];
        } else if instruction == 2 {
            res = instructions[instructions[counter + 1]] * instructions[instructions[counter + 2]];
        } else {
            panic!()
        }
        let address = instructions[counter + 3];
        instructions[address] = res;
        counter += 4;
    }
    instructions
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
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
    fn part1() {
        assert!(run("1,9,10,3,2,3,11,0,99,30,40,50") == "3500,9,10,70,2,3,11,0,99,30,40,50");
        assert!(run("1,0,0,0,99") == "2,0,0,0,99");
        assert!(run("2,3,0,3,99") == "2,3,0,6,99");
        assert!(run("1,1,1,4,99,5,6,0,99") == "30,1,1,4,2,5,6,0,99");
    }
}
