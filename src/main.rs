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

    println!("day {}: {}", args.day_number, call_day_func(args.day_number, args.second_part));
}

fn call_day_func (day_number:u8, second_part:bool) -> u32 {
    let inputfile = format!("day{}_input.txt", day_number);

    let buf_read = BufReader::new(File::open(inputfile).expect("file not found!"));
    let lines = buf_read.lines().map(|x| x.unwrap()).collect();

    match day_number{
            1 => {day1(lines, second_part)},
            2 => {day2(lines, second_part)},
            3 => {day3(lines, second_part)},
            4 => {day4(lines, second_part)},
            _ => {println!("Unsupported day {}", day_number); 0}
    }
}

fn day1 (lines:Vec<String>, second_part:bool) -> u32 {

    let mut calories = Vec::new();
    let mut current = 0;
    for line in lines {
        current = match line.as_str(){
            "" => {calories.push(current); 0},
            _  => {current + line.parse::<u32>().unwrap()}
        }
    }
    calories.sort();
    calories.reverse();
    calories[0..= if second_part {2} else {0}].into_iter().fold(0, |sum, x| sum + x)
}


fn day2 (lines:Vec<String>, second_part:bool) -> u32 {
    // Rock = 1, Paper = 2, Scissors = 3,
    // Lose, draw, win: 0, 3, 6

    let mut total_score = 0;
    for line in lines {
        let (elf_string, me_string) = {let s = line.split(" ").collect::<Vec<&str>>();(s[0],s[1])};
        let elf = i8::try_from("ABC".find(elf_string).unwrap()).unwrap() + 1;
        let (me, points);
        if false == second_part {
            me = i8::try_from("XYZ".find(me_string).unwrap()).unwrap() + 1;
            points = match (elf, me) {
                (elf, me) if (elf - me == 1) || (me - elf == 2) => {0},
                (elf, me) if elf == me => {3},
                (elf, me) if (me - elf == 1) || (elf - me == 2) => {6},
                (_, _) => {panic!("Unexpected elf me pair {} {}", elf, me);},
            };
        } else {
            points = i8::try_from("XYZ".find(me_string).unwrap()).unwrap() * 3;
            me = match (elf, points) {
                (elf, points) if points == 0 => {if elf != 1 {elf - 1} else {3}},
                (elf, points) if points == 3 => {elf},
                (elf, points) if points == 6 => {(elf % 3) + 1},
                (_, _) => {panic!("Unexpected elf me pair {} {}", elf, points);},
            };
        }

        total_score += points as u32 + me as u32;
    }
    total_score
}

fn day3 (lines:Vec<String>, second_part:bool) -> u32 {

    let priority = |x| match x {
        x if matches!(x, 'a'..='z') => {1 + x as u32 - 'a' as u32},
        x if matches!(x, 'A'..='Z') => {27 + x as u32 - 'A' as u32},
        _ => {panic!("Invalid character {}", x)},
    };

    let mut sum = 0;
    if false == second_part {
        for line in lines {
            let compartment1 = &line[..(line.len()/2)];
            let compartment2 = &line[(line.len()/2)..];
            let common:Vec<char> = compartment1.chars().filter(|x| compartment2.contains(*x)).collect(); 
            sum += priority(common[0]);
        }
    } else {
        let mut common:Vec<char> = Vec::<char>::new();
        for (line_count, line) in lines.into_iter().enumerate() {
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
    sum
}

fn day4 (lines:Vec<String>, second_part:bool) -> u32 {
    // Check if all items are in one range or vice versa
    let fully_contained = |v:Vec<u32>| { ({v[0]..=v[1]}.all(|i| {v[2]..=v[3]}.contains(&i))) ||
                                         ({v[2]..=v[3]}.all(|i| {v[0]..=v[1]}.contains(&i)))};
    let any_overlap = |v:Vec<u32>| { ({v[0]..=v[1]}.any(|i| {v[2]..=v[3]}.contains(&i))) ||
                                     ({v[2]..=v[3]}.any(|i| {v[0]..=v[1]}.contains(&i)))};
    let fully_contained = if !second_part {fully_contained} else {any_overlap};

    // Split a line from N1-N2,N3-N4 into a vector of u32
    let split_ints = |line:String| {line.split(",").flat_map(|x| x.split("-")).map(|l| l.parse::<u32>().unwrap()).collect()};
    // Count how many lines are fully contained.
    lines.into_iter().filter(|line| fully_contained(split_ints(line.clone()))).count().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_days() {
        // Results are specific to the specific input files stored in the repo.
        assert_eq!(super::call_day_func(1, false), 71934);
        assert_eq!(super::call_day_func(1, true), 211447);
        assert_eq!(super::call_day_func(2, false), 13268);
        assert_eq!(super::call_day_func(2, true), 15508);
        assert_eq!(super::call_day_func(3, false), 8109);
        assert_eq!(super::call_day_func(3, true), 2738);
        assert_eq!(super::call_day_func(4, false), 507);
        assert_eq!(super::call_day_func(4, true), 897);
    }
}
