use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


/*
Rock     1
Paper    2
Scissors 3

    921 Paper Paper       3+2 => 4605
    146 Paper Rock        0+1 => 146
    140 Paper Scissors    6+3 => 1260
     37 Rock Paper        6+2 => 296
    124 Rock Rock         3+1 => 496
    256 Rock Scissors     0+3 => 768
    369 Scissors Paper    0+2 => 738
    315 Scissors Rock     6+1 => 2205
    192 Scissors Scissors 3+3 => 1152

                                11666
*/

enum Chifumi {
    Rock,
    Paper,
    Scissors,
    Unknown
}

fn chifumi(c: &str) -> Chifumi {
    match c {
        "A" => Chifumi::Rock,
        "B" => Chifumi::Paper,
        "C" => Chifumi::Scissors,

        "X" => Chifumi::Rock,
        "Y" => Chifumi::Paper,
        "Z" => Chifumi::Scissors,

        &_ => Chifumi::Unknown
    }
}

fn shape_score(s: &Chifumi) -> u32 {
    match s {
        Chifumi::Rock => 1,
        Chifumi::Paper => 2,
        Chifumi::Scissors => 3,
        _ => panic!()
    }
}

fn round_score(me: &Chifumi, adv: &Chifumi) -> u32 {
    let mut score = 0;

    score += match (me,adv) {
            (Chifumi::Paper, Chifumi::Rock) => 6,
            (Chifumi::Rock, Chifumi::Scissors) =>  6,
            (Chifumi::Scissors, Chifumi::Paper) => 6,

            (Chifumi::Rock, Chifumi::Rock) => 3,
            (Chifumi::Paper, Chifumi::Paper) => 3,
            (Chifumi::Scissors, Chifumi::Scissors) => 3,

            (_,_) => 0
        };
    score += shape_score(me);
    score
}

fn main() {
    let lines = read_lines("./src/day2/input.txt").unwrap();
    let mut total1 = 0;
    let mut total2 = 0;

    for line in lines {
        let l = line.unwrap();
        let a: Vec<&str> = l.split(' ').collect();

        let c0 = chifumi(&a[0][..]);
        let c1 = chifumi(&a[1][..]);

        /*let _score1 = match (&c1,&c0) {
            (Chifumi::Rock, Chifumi::Rock) => 1 + 3,
            (Chifumi::Rock, Chifumi::Paper) =>  1 + 0,
            (Chifumi::Rock, Chifumi::Scissors) =>  1 + 6,

            (Chifumi::Paper, Chifumi::Paper) => 2 + 3,
            (Chifumi::Paper, Chifumi::Scissors) => 2 + 0,
            (Chifumi::Paper, Chifumi::Rock) => 2 + 6,

            (Chifumi::Scissors, Chifumi::Scissors) => 3 + 3,
            (Chifumi::Scissors, Chifumi::Rock) => 3 + 0,
            (Chifumi::Scissors, Chifumi::Paper) => 3 + 6,
            (_,_) => panic!()
        };*/

        total1 += round_score(&c1, &c0);
        total2 += round_score(&c0, &c1);
    }

    println!("me {}", total1);
    println!("adv {}", total2);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
