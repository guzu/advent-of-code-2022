use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Item {
    Num(i32),
    SubList(Vec<Item>)
}

fn parse(s: &str, i: &mut usize) -> Vec<Item> {
    let mut num = String::new();
    let mut list = Vec::<Item>::new();

    while *i < s.len() {
        let c : char = s.chars().nth(*i).unwrap();
            let mut push_num = || {
                if ! num.is_empty() {
                    let n = num.parse::<i32>().unwrap();
                    list.push(Item::Num(n));
                    num.clear();
                }
            };

            match c {
            '0'..='9' => num.push(c),   // ignore
            ' ' | ',' => push_num(),
            '[' if *i == 0 => (),
            '[' if *i > 0 => {
                    *i += 1;
                    let sublist = parse(s, i);
                    list.push(Item::SubList(sublist));
                },
            ']' => {
                push_num();
                break;
            },
            _ => unimplemented!("unexpected {}", c)
            }

        *i += 1;
    }
    list
}

fn compare(l1 : &Vec<Item>, l2: &Vec<Item>) -> Ordering {
    for (i1, i2) in l1.iter().zip(l2.iter()) {
        let ord = match (i1, i2) {
            (Item::Num(n1), Item::Num(n2)) => {
                n1.cmp(n2)
            },
            (Item::Num(n), Item::SubList(s)) => {
                let tmp = vec![Item::Num(*n)];
                compare(&tmp, s)
            },
            (Item::SubList(s), Item::Num(n)) => {
                let tmp = vec![Item::Num(*n)];
                compare(s, &tmp)
            },
            (Item::SubList(s1), Item::SubList(s2)) => 
                compare(s1, s2),
        };

        if ord != Ordering::Equal {
            return ord;
        }
    }

    l1.len().cmp(&l2.len())
}

/*********************************************************************************
 *
 */

fn main() {
    let mut lines = read_lines("./src/day13/input.txt").unwrap();
    let mut packets = Vec::<Vec::<Item>>::new();
    let divider1 = vec![Item::SubList(vec![Item::Num(2)])];
    let divider2 = vec![Item::SubList(vec![Item::Num(6)])];

    packets.push(divider1);
    packets.push(divider2);

    while let Some(l1) = lines.next() {
        if let Some(l2) = lines.next() {
            let mut i : usize;
            i = 0;
            let list1 = parse(&l1.unwrap(), &mut i);
            i = 0;
            let list2 = parse(&l2.unwrap(), &mut i);

            packets.push(list1);
            packets.push(list2);
        }
        if let Some(l3) = lines.next() {
            assert!(l3.unwrap().is_empty());
        }
    }

    packets.sort_by(|l1, l2| compare(l1, l2));

    let mut answer = 1;
    let divider_match = |l : &Vec<Item>| {
        l.len() == 1 && match l.iter().next() {
            Some(Item::SubList(sl)) => match sl.iter().next() {
                Some(Item::Num(2)) | Some(Item::Num(6)) => sl.len() == 1,
                _ => false,
            },
        _ => false,
        }
    };
    for (i, l) in packets.iter().enumerate() {
        if divider_match(l) {
            answer *= i + 1;
        }

    }
    println!("answer: {}", answer);
    assert_eq!(answer, 21836);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
