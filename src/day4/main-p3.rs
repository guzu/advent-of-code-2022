use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./src/day4/input.txt").unwrap();
    let mut count = 0u32;

    let pairs = lines.map(|l| l.unwrap()
                    .split("-")
                    .map(|p| p.split(",")
                                    .collect()));
    println!("{pairs:?}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
