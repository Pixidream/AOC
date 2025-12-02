use advent_01::solve;
use std::fs::File;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./src/input/example.txt")?;
    let reader = BufReader::new(file);

    let password = solve(reader).map_err(io::Error::other)?;

    println!("Password: {password}");
    Ok(())
}
