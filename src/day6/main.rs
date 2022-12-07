use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

#[cfg(feature = "part1")]
const SIZE: usize = 4;
#[cfg(feature = "part2")]
const SIZE: usize = 14;

fn main() {
    let lines = read_lines("./src/day6/input.txt").unwrap();

    for l in lines {
        let a = l.unwrap();
        let mut n : usize = 0;
        let size = SIZE;

        while a[n..n+size].chars().unique().count() < size {
            n += 1;
        }

        println!("answer: {}", n+size);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
