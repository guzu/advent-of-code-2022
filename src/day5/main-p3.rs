use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./src/day5/input.txt").unwrap();
    let mut stacks = Vec::<Vec::<char>>::new();
    let mut lstacks = Vec::<String>::new();

    for line in lines {
        let l = line.unwrap();
        if l.contains("[") {
            lstacks.push(l.clone());
        }
        else if l.contains("move") {
            //
        }
        else if l.contains("1") {
            // Parse column numbers
            //let _b : Vec<i32> = l
            //      .split(' ').map(|s| s.trim())
            //      .filter(|s| !s.is_empty())
            //      .map(|s| { columns = s.parse().unwrap(); columns })
            //      .collect();
            let ncol = l.split(' ').map(|x| x.trim())          
                .filter(|s| !s.is_empty())
                .last().unwrap().parse::<u32>().unwrap();

            println!("{:?}", ncol);

            // Init tables
            for _i in 0..ncol {
                stacks.push(Vec::<char>::new());
            }

            // Parse stacks
            for s in &lstacks {
                for (idx, _c) in s.match_indices('[') {
                    let c = s.chars().nth(idx + 1).unwrap();
                    stacks[idx/4].push(c);
                }
            }
        }
    }

    println!("{:?}", stacks);
    let mut result = String::new();
    for s in stacks {
        result.push(s[0]);
    }
    println!("{}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
