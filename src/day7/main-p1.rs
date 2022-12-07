use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use itertools::Itertools;

fn main() {
    let lines = read_lines("./src/day7/input.txt").unwrap();
    let mut _indent = String::from("");
    let mut stack = vec![0];
    let mut sizes = Vec::<u32>::new();

    for l in lines {
        let ln = l.unwrap();

        if ln.is_empty() {
            continue;
        }

        if ln.starts_with("$") {
            if ln.starts_with("$ cd ..") {
                //println!("change directory : going down : size {}", stack[0]);
                stack[1] += stack[0];
                sizes.push(stack[0]);
                stack.remove(0);
            }
            else if ln.starts_with("$ cd") {
                //println!("change directory : going up ({})", ln);
                stack.insert(0,0);
            }
            else if ln.starts_with("$ ls") {
                // do nothing
            }
        }
        else {
            let sp : Vec<&str> = ln.split(" ").collect();

            if sp[0] != "dir" {
                let size = sp[0].parse::<u32>().unwrap();
                //println!("{} + {}", stack[0], size);
                stack[0] += size;
            }
        }
    }

    let mut total = 0u32;
    for s in sizes {
        if s < 100_000 {
            total += s;
        }
    }
    println!("answer : {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
