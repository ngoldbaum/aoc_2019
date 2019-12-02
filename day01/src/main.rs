use std::error;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input");

    let mut fuel_requirements: Vec<i64> = Vec::new();

    for line in contents.lines() {
        fuel_requirements.push(line.parse::<i64>()?);
    }

    let mut total_fuel: i64 = 0;

    for req in &fuel_requirements {
        total_fuel += fuel_required(*req).unwrap();
    }

    dbg!(total_fuel);

    let mut total_fuel: i64 = 0;

    for req in &fuel_requirements {
        total_fuel += fuel_required_with_fuel(*req)
    }

    dbg!(total_fuel);

    Ok(())
}

fn fuel_required(mass: i64) -> Option<i64> {
    let third_mass: f64 = (mass as f64) / 3.0;
    let ret = (third_mass.floor() - 2.0) as i64;
    if ret >= 0 {
        return Some(ret);
    }
    return None;
}

fn fuel_required_with_fuel(mass: i64) -> i64 {
    let mut fuel = fuel_required(mass).unwrap();
    let mut total_fuel = fuel;
    while let Some(additional_fuel) = fuel_required(fuel) {
        total_fuel += additional_fuel;
        fuel = additional_fuel;
    }
    total_fuel
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
        assert!(fuel_required(12) == Some(2));
        assert!(fuel_required(14) == Some(2));
        assert!(fuel_required(1969) == Some(654));
        assert!(fuel_required(100756) == Some(33583));
    }

    #[test]
    fn part2() {
        assert!(fuel_required_with_fuel(14) == 2);
        assert!(fuel_required_with_fuel(1969) == 966);
        assert!(fuel_required_with_fuel(100756) == 50346);
    }
}
