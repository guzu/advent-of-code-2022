use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq)]
enum Action {
    Open,
    Move,
}

fn main() {
    let lines = read_lines("./src/day16/input-test.txt").unwrap();
    let mut scan = HashMap::<String,(i32,Vec<String>)>::new();

    // Parse
    for line in lines {
        let l = line.unwrap();
        let tok : Vec<&str> = l.split_whitespace().collect();
        let mut dest = Vec::<String>::new();
        for dst in &tok[9..] {
            dest.push(dst.to_string());
        }
        let str_rate = &tok[4].trim_end_matches(';')[5..];
        let rate = str_rate.parse::<i32>().unwrap();

        scan.insert(tok[1].to_string(), (rate, dest));
    }

    let mut count = 30;
    let valve= scan.get("AA").unwrap();
    let action = Action::Open;

    while count > 0 {
        match action {
            Action::Open => {
                if scan[valve].1 == 0 {
                    // Find another one
                    valve += 1;
                    assert!(valve < scan.len());
                    continue;
                }
                scan[valve].2.
            },
            Action::Move => {

            },
        }
        count -= 1;
    }

    println!("{:?}", scan);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
