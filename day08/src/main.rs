use std::convert::TryFrom;
use std::error;
use std::fs::File;
use std::io::Read;

use ndarray::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input");

    let chars = contents.chars().collect::<Vec<char>>();

    let nlayers = chars.len() / 25 * 6;

    let mut data = Array::<u32, Ix3>::zeros((nlayers, 6, 25));

    for (c, d) in chars.iter().zip(data.iter_mut()) {
        *d = c.to_digit(10).unwrap();
    }

    dbg!(data.slice(s![0, .., ..]));
    dbg!(chars.len());

    let mut min_zeros = 25 * 6;
    let mut index = nlayers + 1;

    for i in 0..nlayers {
        let nzeros = data
            .slice(s![i, .., ..])
            .mapv(|a| u32::try_from(a == 0).unwrap())
            .sum();
        if nzeros < min_zeros {
            min_zeros = dbg!(nzeros);
            index = dbg!(i);
        }
    }

    let sl = data.slice(s![index, .., ..]);

    dbg!(
        sl.mapv(|a| u32::try_from(a == 1).unwrap()).sum()
            * sl.mapv(|a| u32::try_from(a == 2).unwrap()).sum()
    );

    let image = data.fold_axis(Axis(0), 2, |acc, x| {
        if *acc != 2 {
            return *acc;
        } else {
            return *x;
        }
    });

    println!("{}", image);

    Ok(())
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}
