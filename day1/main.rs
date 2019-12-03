use std::env;
use std::fs;

fn convert_mass(mass: f32) -> f32 {
    let fuel: f32 = (mass / 3.0).floor() - 2.0;
    return fuel;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    // let mass = &args[1];
    // let mass: f32 = mass.trim().parse::<f32>().expect("Not a number.");
    // args() returns an iterator. Collect() turns into vector.
    // let fuel: f32 = convert_mass(mass);
    // println!("{}", fuel);
}

pub struct Config {
    pub filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 1 {
            return Err("One argument only");
        }
        let filename = args[1].clone();
        Ok(Config { filename })
    }
}
