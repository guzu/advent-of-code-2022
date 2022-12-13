use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use itertools::Itertools;
//use std::collections::HashSet;

#[derive(Debug)]
enum Operand {
    Old,
    New,
    Num(i32)
}
#[derive(Debug)]
enum Operator {
    Plus,
    Mult,
    Minus,
    Div
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    op: (Operand, Operator, Operand),
    test: (i32, usize, usize), // divisible by X, true dest, false dest
    count: u32
}

fn parse_items(ln : Option<&String>) -> Vec<i32> {
    let v : Vec<i32> = ln.unwrap().split(':').last().expect("").split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect();
    v
}

fn parse_op(ln : Option<&String>) -> (Operand, Operator, Operand) {
    let tok : Vec<&str> = ln.unwrap().trim().split(' ').collect();

    assert_eq!(tok[1], "new");
    assert_eq!(tok[2], "=");

    let op1 = match tok[3] {
        "new" => Operand::New,
        "old" => Operand::Old,
        s => Operand::Num(s.parse::<i32>().unwrap())
    };
    let op2 = match tok[5] {
        "new" => Operand::New,
        "old" => Operand::Old,
        s => Operand::Num(s.parse::<i32>().unwrap())
    };
    let op = match tok[4] {
        "+" => Operator::Plus,
        "*" => Operator::Mult,
        _ => unimplemented!()
    };

    (op1, op, op2)
}

fn parse_test(ln : Option<&String>) -> i32 {
    let n = ln.unwrap().split(' ').last().expect("").parse::<i32>().unwrap();
    n
}

fn parse_if(ln : Option<&String>) -> usize {
    let n = ln.unwrap().split(' ').last().expect("").parse::<u32>().unwrap();
    n as usize
}

fn main() {
    let flines = read_lines("./src/day11/input.txt").unwrap();
    let lines : Vec<String> = flines.map(|l| l.unwrap()).collect();
    let mut monkeys = Vec::<Monkey>::new();

    // parse
    let mut it = lines.iter();
    while let Some(line) = it.next() {
            
        if line.is_empty() {
            continue
        }

        if line.starts_with("Monkey") {
            let monkey = Monkey {
                items: parse_items(it.next()),
                op : parse_op(it.next()),
                test : (parse_test(it.next()),
                        parse_if(it.next()),
                        parse_if(it.next())),
                count : 0 
            };
            monkeys.push(monkey);
        }
    }

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            //println!("monkey {} :", i+1);

            for item in monkeys[i].items.clone() {                
                let op1 = match monkeys[i].op.0 {
                    Operand::Old => item,
                    Operand::Num(n) => n,
                    _ => unimplemented!()
                };
                let op2 = match monkeys[i].op.2 {
                    Operand::Old => item,
                    Operand::Num(n) => n,
                    _ => unimplemented!()
                };
                let new = match monkeys[i].op.1 {
                    Operator::Plus => op1 + op2,
                    Operator::Mult => op1 * op2,
                    _ => unimplemented!()
                } / 3;

                // Test
                let idx = if new % monkeys[i].test.0 == 0 {
                    monkeys[i].test.1
                } else {
                    monkeys[i].test.2
                };

                //println!(" old {} => push {} into {}", item, new, idx);
                monkeys[idx].items.push(new);
                monkeys[i].count += 1;
            }
            monkeys[i].items = Vec::<i32>::new();
        }

        //for (n, m) in monkeys.iter().enumerate() {
        //    println!("Monkey {} (count: {}) : {:?}", n, m.count, m.items);
        //}
    }

    monkeys.sort_by(|a,b| a.count.cmp(&b.count));
    let answer = monkeys.iter().rev().take(2).fold(1, |x,m| x * m.count);
    println!("answer: {:?}", answer);

    assert_eq!(answer, 113232);   // for refactoring
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
