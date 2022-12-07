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
            let a : Vec<&str> = l.split(' ').collect();
            let n = a[1].parse::<u32>().unwrap();
            let src = a[3].parse::<usize>().unwrap() - 1;
            let dst = a[5].parse::<usize>().unwrap() - 1;
            
            for _i in 0..n {
                let x = stacks[src].remove(0);
                stacks[dst].insert(0,x);          
            }
        }
        else if l.contains("1") {
            let _ncol = l.split(' ').map(|x| x.trim())          
                .filter(|s| !s.is_empty())
                .last().unwrap().parse::<u32>().unwrap();
            let ncol = l.split_whitespace()
                .last().unwrap()
                .parse::<u32>().unwrap();

            for _i in 0..ncol {
                stacks.push(Vec::<char>::new());
            }

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
