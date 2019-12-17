use std::fs::File;
use std::io::Read;

fn main() {
    let sequence: Vec<i32> = get_sequence(&get_contents("input"));
    dbg!(fft(&sequence, 100)[..8].to_vec());

    let sequence = repeat10000(get_sequence(&get_contents("input")));
    let offset = 5972877;
    dbg!(fft_fast(&sequence[offset..], 100)[..8].to_vec());
}

fn get_sequence(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect()
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents.trim().to_string()
}

fn get_pattern(row: usize, column: usize) -> i32 {
    match ((column + 1) / (row + 1)) % 4 {
        0 => 0,
        1 => 1,
        2 => 0,
        3 => -1,
        _ => panic!(),
    }
}

fn fft_fast(sequence: &[i32], nphases: usize) -> Vec<i32> {
    let mut iteration = sequence.to_vec().clone();
    for _ in 0..nphases {
        for i in (0..sequence.len() - 1).rev() {
            iteration[i] += iteration[i + 1];
        }
        iteration = iteration.iter().map(|x| x % 10).collect();
    }
    iteration
}

fn fft(sequence: &[i32], nphases: usize) -> Vec<i32> {
    let mut last_iteration = sequence.to_vec().clone();
    let mut ret: Vec<i32> = Vec::new();
    for _ in 0..nphases {
        ret = (0..sequence.len())
            .map(|i| {
                last_iteration
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (j, s)| acc + s * get_pattern(i, j))
                    .abs()
                    % 10
            })
            .collect();
        last_iteration = ret.clone();
    }
    ret
}

fn repeat10000(input: Vec<i32>) -> Vec<i32> {
    let ninput = input.len();
    input
        .into_iter()
        .cycle()
        .take(10000 * ninput)
        .collect::<Vec<i32>>()
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(fft(&get_sequence("12345678"), 1) == get_sequence("48226158"));
        assert!(fft(&get_sequence("12345678"), 2) == get_sequence("34040438"));
        assert!(fft(&get_sequence("12345678"), 3) == get_sequence("03415518"));
        assert!(fft(&get_sequence("12345678"), 4) == get_sequence("01029498"));
        assert!(
            fft(&get_sequence("80871224585914546619083218645595"), 100)[..8].to_vec()
                == get_sequence("24176176")
        );
        assert!(
            fft(&get_sequence("19617804207202209144916044189917"), 100)[..8].to_vec()
                == get_sequence("73745418")
        );
        assert!(
            fft(&get_sequence("69317163492948606335995924319873"), 100)[..8].to_vec()
                == get_sequence("52432133")
        );
        let seq = repeat10000(get_sequence("03036732577212944063491565474664"));
        let offset = 0303673;
        assert!(fft_fast(&seq[offset..], 100)[..8].to_vec() == get_sequence("84462026"));
    }
}
