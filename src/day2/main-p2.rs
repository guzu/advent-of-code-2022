use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Chifumi {
    Rock,
    Paper,
    Scissors,
}
enum End {
    Lose,
    Draw,
    Win,
}

fn chifumi(c: &str) -> Chifumi {
    match c {
        "A" => Chifumi::Rock,
        "B" => Chifumi::Paper,
        "C" => Chifumi::Scissors,
        &_ => panic!()
    }
}

fn end(c: &str) -> End {
    match c {
        "X" => End::Lose,
        "Y" => End::Draw,
        "Z" => End::Win,
        &_ => panic!()
    }
}

fn shape_score(s: &Chifumi) -> u32 {
    match s {
        Chifumi::Rock => 1,
        Chifumi::Paper => 2,
        Chifumi::Scissors => 3,
    }
}

/*
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
}*/

fn main() {
    let lines = read_lines("./src/day2/input.txt").unwrap();
    let mut total = 0;

    for line in lines {
        let l = line.unwrap();
        let a: Vec<&str> = l.split(' ').collect();

        let c0 = chifumi(&a[0][..]);
        let c1 = end(&a[1][..]);

        let score = match (&c0,&c1) {
            (Chifumi::Rock, End::Lose) => 0 + shape_score(&Chifumi::Scissors),
            (Chifumi::Rock, End::Draw) => 3 + shape_score(&Chifumi::Rock), 
            (Chifumi::Rock, End::Win) => 6 + shape_score(&Chifumi::Paper),

            (Chifumi::Paper, End::Lose) => 0 + shape_score(&Chifumi::Rock),
            (Chifumi::Paper, End::Draw) => 3 + shape_score(&Chifumi::Paper),
            (Chifumi::Paper, End::Win) => 6 + shape_score(&Chifumi::Scissors),

            (Chifumi::Scissors, End::Lose) => 0 + shape_score(&Chifumi::Paper),
            (Chifumi::Scissors, End::Draw) => 3 + shape_score(&Chifumi::Scissors),
            (Chifumi::Scissors, End::Win) => 6 + shape_score(&Chifumi::Rock),
        };

        println!("{}", score);
        total += score;
    }

    println!("total {}", total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
