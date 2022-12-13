use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*********************************************************************************
 * For debug
 */
fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        for ch in line.iter() {
            print!("{}", *ch)
        }
        println!();
    }
}

fn print_spmap(map: &Vec<Vec<u32>>) {
    for line in map {
        for ch in line.iter() {
            if *ch == u32::MAX {
                print!("{:>5}", '.')
            } else {
                print!("{:>5}", *ch)
            }
        }
        println!();
    }
}

/*********************************************************************************
 *
 */
fn is_ok(cur : char, prev: char) -> bool {
    match (cur,prev) {
        ('S','S') => true,
        ('S',  _) => false,
        ('a','S') => true,
        (  _,'S') => false,
        ('E','z') => true,
        ('E',  _) => false,
        ('a'..='z','a'..='z') => {
            ((cur as u32) == (prev as u32)+1) || (cur as u32) <= (prev as u32)
        },
        (_,_) => panic!("cur:{} prev:{}", cur, prev)
    }
}

fn walk(map : &Vec<Vec<char>>,
        spmap : &mut Vec<Vec<u32>>,
        trail: &mut Vec<(usize,usize)>,
        x: usize, y: usize,
        prev: char,
        steps: u32) -> Option<u32>
{
    // Check coord
    if y >= map.len() || x >= map[y].len() {
        return None;
    }

    if trail.contains(&(x,y)) {
        return None;
    }

    if steps >= spmap[y][x] {
        return None;
    }

    let cur = map[y][x];

    if !is_ok(cur, prev)
    {
        return None;
    }

    if cur == 'E' {
        println!("Found a solution : length {} {}", steps, trail.len());
        return Some(steps);
    }

    trail.push((x,y));
    spmap[y][x] = steps;

    let mut min = u32::MAX;
    if x > 0 {
        if let Some(n) = walk(map, spmap, trail, x - 1, y, cur, steps+1) {
            min = std::cmp::min(min, n);
        }
    }
    if y > 0 {
        if let Some(n) = walk(map, spmap, trail, x, y - 1, cur, steps+1) {
            min = std::cmp::min(min, n);
        }
    }
    if let Some(n) = walk(map, spmap, trail, x + 1, y, cur, steps+1) {
        min = std::cmp::min(min, n);
    }
    if let Some(n) = walk(map, spmap, trail, x, y + 1, cur, steps+1) {
        min = std::cmp::min(min, n);
    }

    trail.pop();

    if min == u32::MAX {
        None        
    }
    else {
        Some(min)
    }
}

/*********************************************************************************
 *
 */

fn main() {
    let lines = read_lines("./src/day12/input.txt").unwrap();
    let mut map = Vec::<Vec<char>>::new();
    let mut start : (usize,usize) = (0,0);
    let mut trail = Vec::<(usize,usize)>::new();

    // Build map
    for (y,line) in lines.enumerate() {
        let s = line.unwrap().chars().collect::<Vec<char>>();

        if let Some(x) = s.iter().position(|c| *c == 'S') {
            start = (x, y);
        }

        map.push(s);
    }

    let mut spmap = vec![vec![u32::MAX; map[0].len()]; map.len()];
    let res = walk(&mut map, &mut spmap, &mut trail, start.0, start.1, 'S', 0);

    if let Some(steps) = res {
        println!("answer: {:?}", steps);
        assert_eq!(steps, 490);   // for refactoring
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
