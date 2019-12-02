use std::error;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input");

    let mut total_fuel: i64 = 0;

    for line in contents.lines() {
        total_fuel += fuel_required(line.parse::<i64>()?);
    }

    dbg!(total_fuel);

    Ok(())
}

fn fuel_required(mass: i64) -> i64 {
    let third_mass: f64 = (mass as f64) / 3.0;
    return (third_mass.floor() - 2.0) as i64;
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
        assert!(fuel_required(12) == 2);
        assert!(fuel_required(14) == 2);
        assert!(fuel_required(1969) == 654);
        assert!(fuel_required(100756) == 33583);
    }
}
