use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Expr {
    Num(i64),
    SubExpr(String,Op,String),
}

fn eval(hash: &HashMap<String, Expr>, name: &String) -> i64 {
    match hash.get(name) {
    Some(Expr::Num(n)) => *n,
    Some(Expr::SubExpr(name1,op,name2)) => {
        let n1 = eval(hash, name1);
        let n2 = eval(hash, name2);
        match op {
        Op::Add => n1 + n2,
        Op::Sub => n1 - n2,
        Op::Mul => n1 * n2,
        Op::Div => n1 / n2,
        }
    },
    _ => panic!(),
    }
}

fn main() {
    // load file
    let lines = read_lines("./src/day21/input.txt");
    let mut hash : HashMap<String,Expr> = HashMap::<String,Expr>::new();

    for line in lines.unwrap() {
        let l = line.unwrap();
        let mut tok = l.split(':');
        let name = tok.next().unwrap();
        let expr = tok.next().unwrap();

        if let Ok(n) = expr.trim().parse::<i64>() {
            hash.insert(name.to_string(), Expr::Num(n));
        }
        else {
            let mut subexpr = expr.trim().split(' ');
            let name1 = subexpr.next().unwrap();
            let sop   = subexpr.next().unwrap();
            let name2 = subexpr.next().unwrap();

            println!("{}/{}/{}", name1, sop, name2);
            let op = match sop {
            "+" => Op::Add,   
            "-" => Op::Sub,   
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => panic!(""),
            };

            hash.insert(name.to_string(), 
                        Expr::SubExpr(name1.to_string(), op, name2.to_string()));
        }
    }

    println!("{:?}", eval(&hash, &String::from("root")));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
