use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn display_row(row : &Vec<char>) {
        for ch in row {
            print!("{ch}");
        }
        println!();
}

fn get(row: &mut Vec<char>,
        window: ((i32,i32),(i32,i32)),
        x : i32, y : i32) -> char
{
    let ((xmin,ymin),(_xmax,_ymax)) = window;

    //assert!(x < xmin || y < ymin || x > xmax || y > ymax {

    row[(x-xmin)as usize]
}

fn set(row: &mut Vec<char>,
       window: ((i32,i32),(i32,i32)),
       x : i32, y : i32, ch : char)
{
    let ((xmin,ymin),(_xmax,_ymax)) = window;

    row[(x-xmin)as usize] = ch;
}

fn dist_manhattan(x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> u32 {
    let dist_x = x1.max(x2) - x1.min(x2);
    let dist_y = y1.max(y2) - y1.min(y2);

    (dist_x.abs() + dist_y.abs()) as u32
}

fn main() {
    let lines = read_lines("./src/day15/input.txt").unwrap();
    let target : i32 = 2_000_000;
    //let target : i32 = 10;
    let mut sensors = Vec::<((i32,i32),(i32,i32))>::new();
    let mut xmin : i32 = i32::MAX;
    let mut ymin : i32 = i32::MAX;
    let mut xmax : i32 = 0;
    let mut ymax : i32 = 0;

    for l in lines {
        for (sen_x,sen_y,bea_x,bea_y) in l.unwrap() 
                        .split_whitespace()
                        .into_iter()
                        .filter(|s| (**s).find('=') != None)
                        .map(|s| s[2..].trim_end_matches(',').trim_end_matches(':').parse::<i32>().unwrap())
                        .tuple_windows()
        {
            let dist = dist_manhattan(sen_x, sen_y, bea_x, bea_y) as i32;

            sensors.push(((sen_x,sen_y),(bea_x,bea_y)));

            xmin = xmin.min(sen_x - dist);
            ymin = ymin.min(sen_y - dist);

            xmax = xmax.max(sen_x + dist);
            ymax = ymax.max(sen_x + dist);
        }
    }

    //println!("{}", xmax-xmin+1);

    let mut row = vec!['.'; (xmax-xmin+1) as usize];
    let window = ((xmin,ymin),(xmax,ymax));

    for ((x1,y1),(x2,y2)) in sensors.clone() {
        if y1 == target {
            set(&mut row, window, x1, y1, 'S');
        }
        if y2 == target {
            set(&mut row, window, x2, y2, 'B');
        }
    }

    //display_row(&row);

    for x in xmin..xmax {
        for ((sen_x,sen_y),(bea_x,bea_y)) in &sensors {
            let dist_beacon = dist_manhattan(*sen_x, *sen_y, *bea_x, *bea_y) as i32;
            let dist_me = dist_manhattan(*sen_x, *sen_y, x, target) as i32;

            if dist_me <= dist_beacon && get(&mut row, window, x, target) == '.' {
                set(&mut row, window, x, target, '#');
            }
        }
    }

    display_row(&row);

    let count = row.iter().filter(|c| **c == '#').count();
    println!("count : {}", count);
    assert_eq!(count, 5240818);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
