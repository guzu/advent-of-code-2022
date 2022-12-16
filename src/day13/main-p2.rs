use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod day13;

fn main() {
    let mut lines = read_lines("./src/day13/input.txt").unwrap();
    let mut packets = Vec::<Vec::<day13::Item>>::new();
    let divider1 = vec![day13::Item::SubList(vec![day13::Item::Num(2)])];
    let divider2 = vec![day13::Item::SubList(vec![day13::Item::Num(6)])];

    packets.push(divider1);
    packets.push(divider2);

    while let Some(l1) = lines.next() {
        if let Some(l2) = lines.next() {
            let list1 = day13::parse(&l1.unwrap());
            let list2 = day13::parse(&l2.unwrap());

            packets.push(list1);
            packets.push(list2);
        }
        if let Some(l3) = lines.next() {
            assert!(l3.unwrap().is_empty());
        }
    }

    packets.sort_by(|l1, l2| day13::compare(l1, l2));

    let divider_match = |l : &Vec<day13::Item>| {
        l.len() == 1 && match l.iter().next() {
            Some(day13::Item::SubList(sl)) => match sl.iter().next() {
                Some(day13::Item::Num(2)) | Some(day13::Item::Num(6)) => sl.len() == 1,
                _ => false,
            },
        _ => false,
        }
    };
    let answer = packets.into_iter().enumerate()
        .fold(1, |n, x| 
            if divider_match(&x.1) { n * (x.0 + 1) } else { n });
    println!("answer: {}", answer);
    assert_eq!(answer, 21836);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
