use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use itertools::Itertools;
use std::collections::HashSet;

fn move_tail(head : (i32,i32), tail : (i32,i32)) -> (i32,i32) {
    let dist_x = head.0 - tail.0;
    let dist_y = head.1 - tail.1;

    let ret =
        match (dist_x.abs(), dist_y.abs()) {
            // no move
            (0,0) => (tail.0, tail.1),
            (0,1) => (tail.0, tail.1),
            (1,0) => (tail.0, tail.1),
            (1,1) => (tail.0, tail.1),

            // simple move
            (2,0) => (tail.0 + dist_x.signum(), tail.1),
            (0,2) => (tail.0, tail.1 + dist_y.signum()),

            // cross-move
            (2,1) => (tail.0 + dist_x.signum(), head.1),
            (1,2) => (head.0, tail.1 + dist_y.signum()),

            (2,2) => (tail.0 + dist_x.signum(), tail.1 + dist_y.signum()),

            (_,_) => panic!("gni? {} {}", dist_x, dist_y)
        };

    //println!("head{:?} tail{:?} : {:?} => {:?}", head, tail, (dist_x, dist_y), ret);
    ret
}

fn main() {
    let lines = read_lines("./src/day9/input.txt").unwrap();
    let mut snake : Vec<(i32,i32)> = vec![(0,0);10];   // relace 10 by 2 to solve part1
    let mut head : (i32,i32) = (0,0);
    let mut pos = HashSet::<(i32,i32)>::new();

    for ln in lines {
        let s = ln.unwrap();
        let mv : Vec<&str> = s.split(' ').collect();
        let n = mv[1].parse::<i32>().unwrap();

        let mov = match mv[0] {
            "R" => (1,0),
            "L" => (-1,0),
            "U" => (0,1),
            "D" => (0,-1),
            _ => unreachable!()
        };

        (0..n).for_each(|_| {
                snake[0] = (snake[0].0 + mov.0, snake[0].1 + mov.1);
                head = snake[0];
                for i in 1..snake.len() {
                    let tail = move_tail(head, snake[i]);
                    snake[i] = tail;
                    head = tail;
                }
                pos.insert(*snake.last().unwrap());
            });
    }

    println!("answer: {}", pos.len());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
