fn main() {
    let mut num_valid = 0;
    for i in 272091..815432 {
        let digits = i
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        if digits.windows(2).all(|w| w[0] <= w[1]) && digits.windows(2).any(|w| w[0] == w[1]) {
            let match_pairs = digits
                .windows(2)
                .map(|w| w[0] == w[1])
                .collect::<Vec<bool>>();
            if match_pairs[..2] == [true, false]
                || match_pairs[..3] == [false, true, false]
                || match_pairs[1..4] == [false, true, false]
                || match_pairs[2..5] == [false, true, false]
                || match_pairs[3..] == [false, true]
            {
                num_valid += 1;
            }
        }
    }
    dbg!(num_valid);
}
