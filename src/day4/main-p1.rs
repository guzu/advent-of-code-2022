use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn contains(p1 : &[u32], p2 : &[u32]) -> bool {
    p1[0] >= p2[0] && p1[1] <= p2[1]
}

fn main() {
    let lines = read_lines("./src/day4/input.txt").unwrap();
    let mut count = 0u32;

    for line in lines {
        let l : String = line.unwrap().to_string();
        let mut pairs = Vec::<u32>::new();

        for e in l.split(',') {
            let a :Vec<&str> = e.split('-').collect();

            pairs.push(a[0].parse::<u32>().unwrap());
            pairs.push(a[1].parse::<u32>().unwrap());
        }

        if contains(&pairs[0..2], &pairs[2..]) ||
        contains(&pairs[2..], &pairs[0..2]) {
            println!("{:?}", pairs);
            count += 1;
        }
    }
    println!("{count}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
