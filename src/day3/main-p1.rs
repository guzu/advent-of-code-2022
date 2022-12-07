use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let lines = read_lines("./src/day3/input.txt").unwrap();
    let mut sum = 0u32;

    for line in lines {
        let l : String = line.unwrap().to_string();
        let mut set = HashSet::<char>::new();

        assert!(l.len() % 2 == 0);

        let half = l.len()/2;
        //println!("{} {}", &l[0..half], &l[half..]);
        for (_s,c) in l[0..half].char_indices() {
            set.insert(c);
        }
        for (_s,c) in l[half..].char_indices() {
            if set.contains(&c) {
                let mut ascii = c as u32;

                set.remove(&c);
                match c {
                    'a'..='z' => ascii = ascii + 1 - 'a' as u32,
                    'A'..='Z' => ascii = ascii + 27 - 'A' as u32 ,
                    _ => panic!()
                }
                println!("dup found {} => {}", c, ascii);
                sum += ascii;
            }
        }
    }
    println!("==> {}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
