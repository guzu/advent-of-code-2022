use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./src/day1/input.txt").unwrap();
    let mut cal: u32 = 0u32;
    //let mut max = 0u32;
    let mut elfes : Vec<u32> = vec!();

    for line in lines {
        if let Ok(n) = line.unwrap().parse::<u32>() {
            //println!("# {} ", n);
            cal += n;
        }
        else {
            elfes.push(cal);
            cal = 0;
        }
    }
    elfes.push(cal);

    elfes.sort();
    elfes.reverse();

    println!("max: {}", elfes[0]);
    println!("max[3]: {}", elfes[0] + elfes[1] + elfes[2]);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
