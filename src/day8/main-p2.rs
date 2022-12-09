use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use itertools::Itertools;
//use std::collections::HashSet;

fn count_visible(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> i32 {
    let me : u32 = matrix[y][x];
    let mut result = 1;

    //println!("me: {}", me);

    // right
    if x < matrix[y].len() {
        let mut max_pos = matrix[y].len() - 1;
        for (xr, h) in matrix[y][x+1..].iter().enumerate() {
            //println!("  r: {} {}", h, xr);
            max_pos = xr + x + 1;
            if *h >= me {
                break;
            }
        }
        //println!("  right: {}", max_pos-x);
        result *= (max_pos-x) as i32;
    }
    else {
        result *= 0;
    }

    if x > 0 {
        // left
        let mut max_pos = 0;
        for (xl,h) in matrix[y][0..x].iter().rev().enumerate() {
            //println!("  l: {}   {}", h, xl);
            if *h >= me {
                max_pos = x-1-xl;
                break;  
            }
        }
        //println!("  left: {}", x - max_pos);
        result *= (x-max_pos) as i32;
    }
    else {
        result *= 0;
    }

    // top
    if y > 0 {
        let mut max_pos = 0;
        for yt in (0..y).rev() {
            let h = matrix[yt][x];
            //println!("  t: {}", h);
            if h >= me {
                max_pos = yt;
                break;  
            }
        }
        //println!("  top: {}", y-max_pos);
        result *= (y-max_pos) as i32;
    }
    else {
        result *= 0;
    }

    // bottom
    if y < matrix.len()-1 {

        let mut max_pos = matrix.len() - 1;
        for yb in y+1..matrix.len() {
            let h = matrix[yb][x];

            //println!("  b: {}", h);
            if h >= me {
                max_pos = yb;
                break;  
            }
        }
        //println!("  bottom: {}", max_pos-y);
        result *= (max_pos-y) as i32;
    }
    else {
        result *= 0;
    }

    result
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

    #[cfg(feature = "test")]
    loop {
        //println!("{}", count_visible(&matrix, 55, 78));
        println!("{}", count_visible(&matrix, 4, 3));
        return ()
    }

    let mut max = 0;
    for (y,line) in matrix.iter().enumerate() {
        for (x, _val) in line.iter().enumerate() {
            let count = count_visible(&matrix, x, y);
            max = std::cmp::max(max, count);
            //println!("{x},{y} {} ======> {}", matrix[y][x], count);
        }
    }

    println!("answer: {}", max);

    assert_eq!(max, 504000);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
