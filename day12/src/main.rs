use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate lazy_static;
use itertools::Itertools;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"<x=(.+), y=(.+), z=(.+)>").unwrap();
}
fn main() {
    let state = get_contents("input");

    let positions: Vec<[i64; 3]> = state
        .lines()
        .map(|l| {
            let mut position = [0; 3];
            match RE.captures(l) {
                Some(c) => {
                    position[0] = c[1].parse().unwrap();
                    position[1] = c[2].parse().unwrap();
                    position[2] = c[3].parse().unwrap();
                }
                None => panic!(),
            }
            position
        })
        .collect();

    let mut system = System::new(&positions);

    while system.time < 1000 {
        system.print_state();
        system.update();
    }

    system.print_state();
    system.print_energy();
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

#[derive(PartialEq, Debug, Clone)]
struct Moon {
    velocity: [i64; 3],
    position: [i64; 3],
}

#[derive(Debug)]
struct System {
    time: usize,
    moons: Vec<Moon>,
}

impl System {
    fn new(positions: &[[i64; 3]]) -> System {
        let mut ret = System {
            time: 0,
            moons: Vec::new(),
        };
        let velocities = vec![[0; 3]; positions.len()];
        for (p, v) in positions.iter().zip(velocities.iter()) {
            ret.moons.push(Moon {
                position: *p,
                velocity: *v,
            });
        }
        ret
    }

    fn update(&mut self) {
        let moon_copy = self.moons.clone();
        let pairs = moon_copy.iter().enumerate().combinations(2);
        for pair in pairs {
            let (i0, m0) = pair[0];
            let (i1, m1) = pair[1];

            for ax in 0..3 {
                if m0.position[ax] < m1.position[ax] {
                    self.moons[i0].velocity[ax] += 1;
                    self.moons[i1].velocity[ax] -= 1;
                } else if m0.position[ax] > m1.position[ax] {
                    self.moons[i0].velocity[ax] -= 1;
                    self.moons[i1].velocity[ax] += 1;
                }
            }
        }

        self.moons = self
            .moons
            .iter_mut()
            .map(|m| {
                let mut p = m.position;
                let v = m.velocity;

                for ax in 0..3 {
                    p[ax] += v[ax];
                }
                m.position = p;
                m.velocity = v;
                m.clone()
            })
            .collect();
        self.time += 1;
    }

    fn print_energy(&self) {
        let mut tesum = 0;
        println!("Energy after {} steps:", self.time);
        for moon in self.moons.iter() {
            let p = moon.position;
            let v = moon.velocity;

            let pot = p[0].abs() + p[1].abs() + p[2].abs();
            let ke = v[0].abs() + v[1].abs() + v[2].abs();

            let te = pot * ke;

            tesum += te;

            println!("pot: {}; kin: {}; total: {}", pot, ke, te);
        }
        println!("Sum of total energy: {}", tesum);
    }

    fn print_state(&self) {
        println!("After {} steps:", self.time);
        for moon in self.moons.iter() {
            let p = moon.position;
            let v = moon.velocity;
            println!(
                "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
                p[0], p[1], p[2], v[0], v[1], v[2]
            );
        }
    }
}
