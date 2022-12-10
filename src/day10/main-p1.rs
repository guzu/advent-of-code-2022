use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use itertools::Itertools;
//use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32)
}

#[derive(Debug)]
struct Pipeline {
    count : u32,
    instr : Instruction,
    reg_x : i32,
}

fn main() {
    let mut lines = read_lines("./src/day10/input.txt").unwrap();
    let mut pipeline : Pipeline = Pipeline { count: 0, instr: Instruction::Noop, reg_x: 1 };
    let mut sum = 0i32;

    // parse
    for tick in 1..300 {

        if pipeline.count == 0 {
            match pipeline.instr {
                Instruction::Noop => (),
                Instruction::Addx(n) => pipeline.reg_x += n,
            }
        
            // parse new instruction
            if let Some(line) = lines.next() {
                let s = line.unwrap();
                let i : Vec<&str> = s.split(' ').collect();
                match i[0] {
                    "noop" => {
                        pipeline.instr = Instruction::Noop;
                        pipeline.count = 0;
                    }
                    "addx" => {
                        pipeline.instr = Instruction::Addx(i[1].parse::<i32>().unwrap());
                        pipeline.count = 1;
                    }
                    _ => panic!("Bus error")
                }
                //println!("====> tick: {}   pipeline: {:?}", tick, pipeline);
            }
            else { break; }
        }
        else {
            pipeline.count -= 1;
            //println!("====> tick: {}   pipeline: {:?}", tick, pipeline);
        }

        match tick {
            20 | 60 | 100 | 140 | 180 | 220 => {
                println!("=======> tick: {}  signal: {}", tick, tick * pipeline.reg_x);
                sum += tick * pipeline.reg_x;
            },
            _=> ()
        }
    }

    println!("answer part1: {}", sum);
    assert_eq!(sum, 12460);   // in case of refactoring
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
