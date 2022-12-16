use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn display_map(map : &Vec<Vec<char>>, xmin:usize, _ymin:usize, xmax:usize, ymax:usize) {
    // Starts at 0 because sand is starting 0...
    for (i, row) in map[0..=ymax+1].iter().enumerate() {
        print!("{:>5} ", i);
        for ch in &row[xmin-1..xmax+1] {
            print!("{ch}");
        }
        println!();
    }
}

fn draw_segment(map : &mut Vec<Vec<char>>, begin : (usize,usize), end : (usize,usize)) {
    if begin.0 != end.0 {
        assert_eq!(begin.1, end.1);
        let y = begin.1;
        for x in begin.0.min(end.0)..=begin.0.max(end.0) {
            map[y][x] = '#';
        }
    }
    else if begin.1 != end.1 {
        assert_eq!(begin.0, end.0);
        let x = begin.0;
        for y in begin.1.min(end.1)..=begin.1.max(end.1) {
            map[y][x] = '#';
        }
    }
}

fn main() {
    let lines = read_lines("./src/day14/input.txt").unwrap();
    let mut xmin : usize = usize::MAX;
    let mut ymin : usize = usize::MAX;
    let mut xmax : usize = 0;
    let mut ymax : usize = 0;
    let mut segments = Vec::<((usize,usize),(usize,usize))>::new();

    for l in lines {
        for (s1, s2) in l.unwrap() 
                        .split_whitespace()
                        .into_iter()
                        .filter(|s| !(**s).eq("->"))
                        .tuple_windows()
        {
            let c1 : Vec<usize> = s1.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
            let c2 : Vec<usize> = s2.split(',').map(|x| x.parse::<usize>().unwrap()).collect();

            xmin = xmin.min(c1[0].min(c2[0]));
            ymin = ymin.min(c1[1].min(c2[1]));

            xmax = xmax.max(c1[0].max(c2[0]));
            ymax = ymax.max(c1[1].max(c2[1]));

            segments.push( ((c1[0],c1[1]), (c2[0],c2[1])) );
        }
    }

    let row = vec!['.';xmax+2];
    let mut map = vec![row; ymax+2];

    for (begin, end) in segments {
        draw_segment(&mut map, begin, end);
    }

    //display_map(&map, xmin, ymin, xmax, ymax);

    let mut sand : (usize, usize) = (500,0);
    let mut count = 0;
    loop {
        let prev = sand;
        sand.1 += 1;

        if sand.1 >= map.len() {
            break;
        }

        match map[sand.1][sand.0] {
            '.' => {
                map[prev.1][prev.0] = '.';
                map[sand.1][sand.0] = '+';
                },
            'o' | '#' => {
                // try left
                match map[sand.1][sand.0-1] {
                    '.' => {
                        map[prev.1][prev.0] = '.';
                        sand.0 -= 1;
                        map[sand.1][sand.0] = 'o';
                    },
                    'o' | '#' => {
                        // try right
                        match map[sand.1][sand.0+1] {
                            '.' => {
                                map[prev.1][prev.0] = '.';
                                sand.0 += 1;
                                map[sand.1][sand.0] = '+';
                            },
                            'o' | '#' => {
                                map[prev.1][prev.0] = 'o';
                                sand = (500,0);
                                count += 1;
                            },
                            _ => panic!(),
                        }
                    },
                    _ => panic!(),
                }
            },
            _ => panic!()
        }
        //display_map(&map, xmin, ymin, xmax, ymax);
    }

    display_map(&map, xmin, ymin, xmax, ymax);

    println!("count: {}", count);
    assert_eq!(count, 808);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
