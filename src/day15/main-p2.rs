use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn display_map(map : &Vec<Vec<char>>, _xmin:i32, ymin:i32, _xmax:i32, _ymax:i32) {
    for (i, row) in map.iter().enumerate() {
        print!("{:>5} ", ymin + i as i32);
        for ch in row {
            print!("{ch}");
        }
        println!();
    }
}

fn get(map: &mut Vec<Vec<char>>,
        window: ((i32,i32),(i32,i32)),
        x : i32, y : i32) -> char
{
    let ((xmin,ymin),(_xmax,_ymax)) = window;

    map[(y-ymin) as usize][(x-xmin)as usize]
}

fn set(map: &mut Vec<Vec<char>>,
       window: ((i32,i32),(i32,i32)),
       x : i32, y : i32, ch : char)
{
    let ((xmin,ymin),(xmax,ymax)) = window;

    if x < xmin || y < ymin || x > xmax || y > ymax {
        return;
    }

    map[(y-ymin) as usize][(x-xmin)as usize] = ch;
}

fn dist_manhattan(x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> u32 {
    let dist_x = x1.max(x2) - x1.min(x2);
    let dist_y = y1.max(y2) - y1.min(y2);

    (dist_x.abs() + dist_y.abs()) as u32
}

fn dist_manhattan_tuple(x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> (u32,u32) {
    let dist_x = x1.max(x2) - x1.min(x2);
    let dist_y = y1.max(y2) - y1.min(y2);

    (dist_x.abs() as u32, dist_y.abs() as u32)
}

fn main() {
    let lines = read_lines("./src/day15/input.txt").unwrap();
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

    xmin = -1;
    ymin = -1;

    //let maxxx = 20;   // for input-test.txt
    let maxxx = 4_000_000;

    if false {
        let row = vec!['.'; (xmax-xmin+1) as usize];
        let mut map = vec![row; (ymax-ymin+1) as usize];

        let window = ((xmin,ymin),(xmax,ymax));
        
        for ((x1,y1),(x2,y2)) in sensors.clone() {
            set(&mut map, window, x1, y1, 'S');
            set(&mut map, window, x2, y2, 'B')
        }
        display_map(&map, xmin, ymin, xmax, ymax);
    }
    
    xmax = maxxx;

    for y in 0..maxxx {
        let mut x = 0;

        while x <= xmax {
            let mut len = 0i32;
            let mut found = false;

            for ((sen_x,sen_y),(bea_x,bea_y)) in &sensors {
                let dist_beacon = dist_manhattan(*sen_x, *sen_y, *bea_x, *bea_y) as i32;
                let dist_me = dist_manhattan(*sen_x, *sen_y, x, y) as i32;

                if dist_me <= dist_beacon {
                    let (dist_x, dist_y) = dist_manhattan_tuple(*sen_x, *sen_y, x, y);

                    if x < *sen_x {
                        len +=  dist_x as i32 + (dist_beacon - dist_y as i32);
                    }
                    else if x >= *sen_x {
                        len += dist_beacon - dist_me;
                    }

                    if x + len > maxxx {
                        len = maxxx - x;
                    }

                    found = true;
                    break;
                }
            }

            if !found {
                println!("x:{} y:{} => {}", x, y, x as u64 * 4_000_000 as u64 + y as u64);
            }
    
            x += len + 1;
        }
    }

    //display_map(&map, xmin, ymin, xmax, ymax);

    //assert_eq!(count, 13213086906101);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
