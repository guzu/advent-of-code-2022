use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

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

fn parse(filename : &str, hash: &mut HashMap<String, Expr>) {
    // load file
    let lines = read_lines(filename);

    // Parsing
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

            let op = match sop {
                "+" => Op::Add,   
                "-" => Op::Sub,   
                "*" => Op::Mul,
                "/" => Op::Div,
                _ => panic!(),
            };

            hash.insert(name.to_string(), 
                        Expr::SubExpr(name1.to_string(), op, name2.to_string()));
        }
    }


}

fn eval(hash: &HashMap<String, Expr>, name: &String) -> Option<i64> {
    if name == "humn" {
        return None;
    }

    match hash.get(name) {
    Some(Expr::Num(n)) => Some(*n),
    Some(Expr::SubExpr(name1,op,name2)) => {
        if let Some(n1) = eval(hash, name1) {
            if let Some(n2) = eval(hash, name2) {
                Some(match *op {
                    Op::Add => n1 + n2,
                    Op::Sub => n1 - n2,
                    Op::Mul => n1 * n2,
                    Op::Div => n1 / n2,
                })
            }
            else {
                None
            }
        }
        else {
            None
        }
    },
    _ => panic!(),
    }
}

fn resolv(hash: &HashMap<String, Expr>, val: i64, name: &String) -> Option<i64> {
    if name == "humn" {
        return Some(val);
    }

    match hash.get(name) {
        Some(Expr::SubExpr(lname,op,rname)) => {
            let left = eval(&hash, lname);
            let right = eval(&hash, rname);

            match (left,right) {
                (Some(x),None) => {
                    let newval = match *op {
                        Op::Add => val - x,     // val = x + ?  => ? = val-x
                        Op::Sub => -(val - x),  // val = x - ?  => ? = -(val-x)
                        Op::Mul => val / x,     // val = x * ?  => ? = val/x
                        Op::Div => x / val,     // val = x / ?  => ? = x/val
                    };
                    return resolv(hash, newval, rname);
                },
                (None,Some(x)) => {
                    let newval = match *op {
                        Op::Add => val - x,     // val = ? + x  => ? = val-x
                        Op::Sub => val + x,     // val = ? - x  => ? = val + x
                        Op::Mul => val / x,     // val = ? * x  => ? = val/x
                        Op::Div => val * x,     // val = ? / x  => ? = val*x
                    };
                    return resolv(hash, newval, lname);
                },
                _ => unreachable!()
            }
        },
        _ => panic!(),
    }
}

fn main() {
    let mut hash : HashMap<String,Expr> = HashMap::<String,Expr>::new();

    parse("./src/day21/input.txt", &mut hash);

    let (name,val) = match hash.get(&String::from("root")) {
        Some(Expr::SubExpr(n1,_,n2)) => {
            if let Some(n) = eval(&hash, n1) {
                //println!("left expression evaluate to {}", n);
                (n2, n)
            }
            else if let Some(n) = eval(&hash, n2) {
                //println!("right expression evaluate to {}", n);
                (n1, n)
            }
            else {
                panic!()
            }
        }
        _ => panic!()
    };

    if let Some(n) = resolv(&hash, val, name) {
        println!("answer : {}", n);
        assert_eq!(n, 3375719472770);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
