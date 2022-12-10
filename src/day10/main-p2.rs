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

fn print_crt(x: usize) {
    let mut row = ['.'; 40];
    row[x] = '#';

    let s : String = row.iter().collect();
    println!("{}", s);
}

fn print_sprite(x: usize) {
    let mut row = ['.'; 40];

    row[x-1] = '#';
    row[x]   = '#';
    row[x+1] = '#';

    let s : String = row.iter().collect();
    println!("{}", s);
}

fn main() {
    let mut lines = read_lines("./src/day10/input.txt").unwrap();
    let mut pipeline : Pipeline = Pipeline { count: 0, instr: Instruction::Noop, reg_x: 1 };

    // parse
    for clock in 0..800 {
        let cycle = clock / 2 + 1;
        let up = clock % 2 == 0;

        //println!("cycle: {} {} ", cycle, if up {"up"}else{"down"} );

        if up {
            let crt_x = (clock/2) % 40 as i32;
            let sprite = pipeline.reg_x as i32;

            // Draw on CRT

            // if sprite >= 1 && sprite <= 39 {
            //     print_sprite(sprite);
            // }
            // print_crt(crt_x);

            if crt_x >= sprite-1 && crt_x <= sprite+1 {
                //println!("{:>90}", '#');
                print!("#");
            }
            else {
                //println!("{:>90}", '.');
                print!(".");
            }
            if (crt_x+1) % 40 == 0 { println!() }

            if pipeline.count == 0 {
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
                }
                else { break; }
            }
            else {
                pipeline.count -= 1;
            }
        }
        else {
            if pipeline.count == 0 {
                match pipeline.instr {
                    Instruction::Noop => (),
                    Instruction::Addx(n) => pipeline.reg_x += n,
                }
            }            
            //println!("====> cycle: {}   pipeline: {:?}", cycle, pipeline);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
