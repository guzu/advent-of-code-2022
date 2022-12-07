use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use itertools::Itertools;

fn get_val(c: char) -> u32 {
    let mut ascii = c as u32;
    match c {
        'a'..='z' => ascii = ascii + 1 - 'a' as u32,
        'A'..='Z' => ascii = ascii + 27 - 'A' as u32 ,
        _ => panic!()
    }
    ascii
}

fn main() {
    let lines = read_lines("./src/day3/input.txt").unwrap();
    let mut sum = 0u32;

    for (line1, line2, line3) in lines.tuples() {
        let mut map = HashMap::<char,u32>::new();

        for line in [ line1, line2, line3 ] {
            let l : String = line.unwrap().to_string();

            for c in l[0..].chars().sorted().unique() {
                map.entry(c)
                    .and_modify(|x| {
                        *x += 1; if *x == 3 { sum += get_val(c); } })
                    .or_insert(1);
            }    
        }
    }
    println!("==> {}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
