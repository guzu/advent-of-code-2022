use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

fn main() {
    let lines = read_lines("./src/day6/input.txt").unwrap();

    for l in lines {
        let a = l.unwrap();
        let mut n : usize = 0;
        let size = 14 as usize;

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
