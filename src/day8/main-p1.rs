use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use itertools::Itertools;
use std::collections::HashSet;

fn count_visible(matrix: &Vec<Vec<u32>>, set: &mut HashSet<(i32,i32)>, transposed: bool) {
    let mut max;

    let coord = | x: i32,y: i32| -> (i32,i32) {
        if transposed {
            (y,x)
        }
        else {
            (x,y)
        }
    };

    // lines
    let mut y = 0i32;
    for line in matrix.iter() {
        let mut x;

        // from left
        x = 0;
        max = line[0];
        set.insert(coord(x,y));
        for digit in line {
            if *digit > max {
                set.insert(coord(x,y));
                max = *digit;
            }
            x += 1;
        }

        // from right
        x = (line.len()-1) as i32;
        max = line[line.len()-1];
        set.insert(coord(x,y));
        for digit in line.iter().rev() {
            if *digit > max {
                set.insert(coord(x,y));
                max = *digit;
            }
            x -= 1;
        }

        y += 1;
    }
}

fn main() {
    let lines = read_lines("./src/day8/input.txt").unwrap();
    let mut matrix = Vec::<Vec::<u32>>::new();

    // parse
    for line in lines {
        let s = line.unwrap();

        let mut ln = Vec::<u32>::new();
        for digit in s.chars().enumerate() {
            ln.push(digit.1 as u32 - 48);
        }
        matrix.push(ln);
    }

    let mut set = HashSet::<(i32,i32)>::new();
    count_visible(&matrix, &mut set, false);
    matrix = transpose(matrix);
    count_visible(&matrix, &mut set, true);

    println!("{}", set.len());
}

fn transpose(m: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    t
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
