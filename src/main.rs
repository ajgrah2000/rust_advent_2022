use argh::FromArgs;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(FromArgs)]
/// Run selected advent of code functions.
struct AdventArgs {
    /// name of the day 
    #[argh(positional)]
    day_number: u8,

    /// run the second part for the day.
    #[argh(switch, short='s')]
    second_part: bool,
}

fn main() {
    let args:AdventArgs = argh::from_env();

    let inputfile = format!("day{}_input.txt",args.day_number);
    match args.day_number{
            1 => {day1(inputfile, args.second_part);},
            2 => {day2(inputfile, args.second_part);},
            3 => {day3(inputfile, args.second_part);},
            _ => {println!("Unsupported day {}", args.day_number);}
    }
}

fn day1 (filename:String, second_part:bool) {
    let buf_read = BufReader::new(File::open(filename).expect("file not found!"));

    let mut calories = Vec::new();
    let mut current = 0;
    for line in buf_read.lines().map(|x| x.unwrap()) {
        current = match line.as_str(){
            "" => {calories.push(current); 0},
            _  => {current + line.parse::<i32>().unwrap()}
        }
    }
    calories.sort();
    calories.reverse();
    println!("{}", calories[0..= if second_part {2} else {0}].into_iter().fold(0, |sum, x| sum + x));
}


fn day2 (filename:String, second_part:bool) {
    // Rock = 1, Paper = 2, Scissors = 3,
    // Lose, draw, win: 0, 3, 6

    let buf_read = BufReader::new(File::open(filename).expect("file not found!"));

    let mut total_score = 0;
    for line in buf_read.lines().map(|x| x.unwrap()) {
        let s:Vec<&str> = line.split(" ").collect();
        let elf = i8::try_from("ABC".find(s[0]).unwrap()).unwrap() + 1;
        let (me, points);
        if false == second_part {
            me = i8::try_from("XYZ".find(s[1]).unwrap()).unwrap() + 1;
            points = match (elf, me) {
                (elf, me) if (elf - me == 1) || (me - elf == 2) => {0},
                (elf, me) if elf == me => {3},
                (elf, me) if (me - elf == 1) || (elf - me == 2) => {6},
                (_, _) => {panic!("Unexpected elf me pair {} {}", elf, me);},
            };
        } else {
            points = i8::try_from("XYZ".find(s[1]).unwrap()).unwrap() * 3;
            me = match (elf, points) {
                (elf, points) if points == 0 => {if elf != 1 {elf - 1} else {3}},
                (elf, points) if points == 3 => {elf},
                (elf, points) if points == 6 => {(elf % 3) + 1},
                (_, _) => {panic!("Unexpected elf me pair {} {}", elf, points);},
            };
        }

        total_score += points as u32 + me as u32;
    }
    println!("{}", total_score);
}

fn day3 (filename:String, second_part:bool) {
    // Rock = 1, Paper = 2, Scissors = 3,
    // Lose, draw, win: 0, 3, 6

    let buf_read = BufReader::new(File::open(filename).expect("file not found!"));

    let priority = |x| match x {
        x if matches!(x, 'a'..='z') => {1 + x as u32 - 'a' as u32},
        x if matches!(x, 'A'..='Z') => {27 + x as u32 - 'A' as u32},
        _ => {panic!("Invalid character {}", x)},
    };

    let mut sum = 0;
    if false == second_part {
        for line in buf_read.lines().map(|x| x.unwrap()) {
            let compartment1 = &line[..(line.len()/2)];
            let compartment2 = &line[(line.len()/2)..];
            let common:Vec<char> = compartment1.chars().filter(|x| compartment2.contains(*x)).collect(); 
            sum += priority(common[0]);
        }
    } else {
        let mut common:Vec<char> = Vec::<char>::new();
        for (line_count, line) in buf_read.lines().map(|x| x.unwrap()).enumerate() {
            common = if line_count % 3 == 0 {
                         line.chars().collect()
                     } else {
                         line.chars().filter(|x| common.contains(x)).collect()
                     };
            if line_count % 3 == 2 {
                sum += priority(common[0]);
            }
        }
    }
    println!("{}", sum);
}


/* Some initial attempts */
fn _day2_long (filename:String, second_part:bool) {
    #[derive(Copy,Clone, PartialEq)]
    enum RPS {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }

    let buf_read = BufReader::new(File::open(filename).expect("file not found!"));

    let mut total_score = 0;
    for line in buf_read.lines().map(|x| x.unwrap()) {
        let s:Vec<&str> = line.split(" ").collect();

        // Convert to enum
        let rps_match = |allocation:&str, input| {
            let expected:Vec<String> = allocation.chars().collect::<Vec<char>>().into_iter().map(|x| x.to_string()).collect();
            match input {
                s if s == expected[0] => {RPS::Rock},
                s if s == expected[1] => {RPS::Paper},
                s if s == expected[2] => {RPS::Scissors},
                _ => panic!("Unexpected string {} must be from {}", input, allocation),

            }};

        let me;
        let points;
        let elf = rps_match("ABC", s[0]);
        if second_part {
            points = match s[1] {
                "X" => {0},
                "Y" => {3},
                "Z" => {6},
                _ => panic!("Unexpected string {}", s[1]),
            };
            me = match (elf, points) {
                (elf, points) if points == 3 => {elf}, // Draw
                (elf, points) if points == 0 => {
                    match elf {
                        RPS::Rock => {RPS::Scissors},
                        RPS::Paper => {RPS::Rock},
                        RPS::Scissors => {RPS::Paper},
                    }}, 
                (elf, points) if points == 6 => {
                    match elf {
                        RPS::Rock => {RPS::Paper},
                        RPS::Paper => {RPS::Scissors},
                        RPS::Scissors => {RPS::Rock},
                    }}, 
                (_, _) => {panic!("Unexpected elf point combination");},
            }

        } else {
            me = rps_match("XYZ", s[1]);

            points = match (elf, me) {
                (elf, me) if elf == me => {3},
                (elf, me) if elf == RPS::Rock && me == RPS::Paper => {6},
                (elf, me) if elf == RPS::Scissors && me == RPS::Rock => {6},
                (elf, me) if elf == RPS::Paper && me == RPS::Scissors => {6},
                _ => {0},

            };
        }
        total_score += points as u32 + me as u32;
    }
    println!("{}", total_score);
}
