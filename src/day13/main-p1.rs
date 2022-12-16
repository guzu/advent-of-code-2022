use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;

mod day13;

fn main() {
    let mut lines = read_lines("./src/day13/input.txt").unwrap();
    let mut idx = 1;
    let mut sum = 0;
    //assert_eq!(answer, 113232);   // for refactoring

    while let Some(l1) = lines.next() {
        if let Some(l2) = lines.next() {
            let mut i : usize;
            i = 0;
            let list1 = day13::parse(&l1.unwrap(), &mut i);
            i = 0;
            let list2 = day13::parse(&l2.unwrap(), &mut i);
            let ord = day13::compare(&list1, &list2);
            match ord {
                Ordering::Equal | Ordering::Less => sum += idx,
                _ => (),
            }
            println!("{:?}", ord);
            idx += 1;
        }
        if let Some(l3) = lines.next() {
            assert!(l3.unwrap().is_empty());
        }
    }

    println!("answer: {}", sum);
    assert_eq!(sum, 4734);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
