use argh::FromArgs;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

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

fn call_day_func (day_number:u8, second_part:bool) -> String {
    let inputfile = format!("day{}_input.txt", day_number);

    let buf_read = BufReader::new(File::open(inputfile).expect("file not found!"));
    let lines = buf_read.lines().map(|x| x.unwrap()).collect();

    match day_number{
            1 => {format!("{}", day1(lines, second_part))},
            2 => {format!("{}", day2(lines, second_part))},
            3 => {format!("{}", day3(lines, second_part))},
            4 => {format!("{}", day4(lines, second_part))},
            5 => {day5(lines, second_part)},
            6 => {format!("{}", day6(lines, second_part))},
            7 => {format!("{}", day7(lines, second_part))},
            8 => {format!("{}", day8(lines, second_part))},
            _ => {format!("Unsupported day {}", day_number)}
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


#[allow(dead_code)]
fn day2_old (lines:Vec<String>, second_part:bool) -> u32 {
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

// Alternate implementation, trying to make it clearer by using enumerations.
fn day2(lines:Vec<String>, second_part:bool) -> u32 {
    #[derive(PartialEq, Clone, Copy)]
    enum Hand { Rock = 1, Paper = 2, Scissors = 3 }
    #[derive(PartialEq, Clone, Copy)]
    enum Score { Loose = 0, Draw = 3, Win = 6 }

    let hand = vec![Hand::Rock, Hand::Paper, Hand::Scissors];
    let point = vec![Score::Loose, Score::Draw, Score::Win];

    // 'cycle' repeats the itertaor, so it never ends.
    let winning_hand = |other| {*hand.iter().cycle().nth(hand.iter().position(|x| *x == other).unwrap() + 1).unwrap()};
    let loosing_hand = |other| {winning_hand(winning_hand(other))}; //Loosing hand is the hand that 'beats' the winning hand

    let mut total_score = 0;
    for line in lines {

        let (elf_string, me_string) = {let s = line.split(" ").collect::<Vec<&str>>();(s[0],s[1])};

        let elf = hand["ABC".find(elf_string).unwrap() as usize];
        let (points, me);
        if false == second_part {
            me  = hand["XYZ".find(me_string).unwrap() as usize];
            points = match (elf, me) {
                (elf, me) if winning_hand(elf) == me => {Score::Win},
                (elf, me) if elf == me => {Score::Draw},
                (elf, me) if elf == winning_hand(me) => {Score::Loose},
                (_, _) => {panic!("Unexpected elf me pair {} {}", elf as u8, me as u8);},
            };
        } else {
            points  = point["XYZ".find(me_string).unwrap() as usize];
            me = match (elf, points) {
                (elf, points) if points == Score::Loose => {loosing_hand(elf)},
                (elf, points) if points == Score::Draw => {elf},
                (elf, points) if points == Score::Win => {winning_hand(elf)},
                (_, _) => {panic!("Unexpected elf me pair {} {}", elf as u8, points as u8);},
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
    let mut iter = lines.into_iter();
    while let Some(line) = iter.next() {
        let mut common:Vec<char>;
        if false == second_part {
            let compartment1 = &line[..(line.len()/2)];
            let compartment2 = &line[(line.len()/2)..];
            common = compartment1.chars().filter(|x| compartment2.contains(*x)).collect(); 
        } else {
            common = line.chars().collect();
            // Filter common characters from the 2nd & 3rd lines
            common = iter.next().unwrap().chars().filter(|x| common.contains(x)).collect();
            common = iter.next().unwrap().chars().filter(|x| common.contains(x)).collect();
        }
        sum += priority(common[0]);
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

fn day5 (lines:Vec<String>, second_part:bool) -> String {
    let iter = lines.clone().into_iter();

    // Get the initial layout
    let mut boxes = iter.take_while(|v| v != "").clone().collect::<Vec<String>>().into_iter().rev();
    let mut stack_positions = Vec::<VecDeque::<char>>::new();
    let index_line = boxes.next().unwrap();

    // Get the indicies of the stack numbers
    let indicies = index_line.chars().into_iter().enumerate()
                            .filter_map(|(index, x)| if x != ' ' {Some(index)} else {None});
    for index in indicies {
        stack_positions.push(boxes.clone().filter_map(|b| {let x = b.chars().nth(index).unwrap(); 
                                                      if x != ' ' {Some(x)} else {None}}).collect());
    }

    // Get the moves
    let mut moves_iter = lines.into_iter().skip_while(|v| v != "");
    moves_iter.next(); // Ignore the blank line

    while let Some(m) = moves_iter.next() {
        let move_split = m.split_whitespace().collect::<VecDeque<&str>>();
        // move N from A to B -> numbers are at indexes 1, 3, 5
        let move_numbers = vec![1,3,5].into_iter()
                                      .map(|i| move_split[i].parse::<usize>()
                                      .unwrap()).collect::<Vec<usize>>();
        if !second_part {
            // Move one boxes at a time
            for _i in 0..move_numbers[0] {
                let removed = stack_positions[move_numbers[1] - 1].pop_back().unwrap();
                stack_positions[move_numbers[2] - 1].push_back(removed);
            }
        } else {
            // Move multiple boxes at a time
            let last_index = stack_positions[move_numbers[1]-1].len();
            let mut removed = stack_positions[move_numbers[1] - 1]
                                    .drain((last_index-move_numbers[0])..last_index)
                                    .collect::<VecDeque<char>>();
            stack_positions[move_numbers[2] - 1].append(&mut removed);
        }
    }

    stack_positions.into_iter().fold("".to_string(), |current, t| format!("{}{}",current,t.back().unwrap()))
}

fn day6 (lines:Vec<String>, second_part:bool) -> u32 {
    let line = &lines[0]; // There's only one line
    let distinct = if !second_part {4} else {14};

    for i in 0..=(line.len() - distinct) {
        // Check if range of characters are a unique set.
        if line[i..i+distinct].chars()
                .fold(HashSet::new(), |mut h, c| {h.insert(c);h}).len() == distinct {
            return (i + distinct) as u32;
        }
    }
    0
}

fn day7 (lines:Vec<String>, second_part:bool) -> u32 {

    let mut current_path = VecDeque::<String>::new();
    // Hash of 'full path, total size (including sub directories)
    let mut dir_hash = HashMap::new();
    for line in lines {
        let (cd_token, ls_token, dir_token) = ("$ cd ", "$ ls", "dir ");
        match line {
            line if line.contains(cd_token) => {
                let dir = &line[cd_token.len()..];
                match dir { // Push/pop/reset current path
                    dir if dir == ".." => {current_path.pop_back();},
                    dir if dir == "/" =>  {current_path = VecDeque::from(["".to_string()]);},
                    _ =>  { current_path.push_back(dir.to_string());},
                }
            }
            line if line.contains(ls_token) => {},  // Do nothing for 'ls'
            line if line.contains(dir_token) => {}, // Ignore 'directories'
            // Input is well formed, the default output is always the file listing
            _ => {let size_file = line.split_whitespace().collect::<Vec<&str>>();
                  let size = size_file[0].parse::<u32>().unwrap();

                  // Accumulate the size for current path and all parents.
                  current_path.clone().into_iter().fold("".to_string(), |parent, k| {
                      let current_size = dir_hash.entry(format!("{}/{}",parent,k)).or_insert(0);
                      *current_size += size;
                      parent + "/" + &k
                  });
            },
        }
    }

    if !second_part {
        let mut total = 0;
        for (_, data) in dir_hash {
            // Only total directories over 100000
            if data  <= 100000 {
                total += data;
            }
        }
        total
    } else {
        let max_space = 70000000;
        let required_unused_space = 30000000;
        let current_unused = max_space - dir_hash.get("/").unwrap();
        let need_to_free = required_unused_space  - current_unused;
        dir_hash.into_iter().filter(|(_,d)| d > &need_to_free).fold(max_space, |min,(_,d)| std::cmp::min(min,d))
    }
}

fn day8 (lines:Vec<String>, second_part:bool) -> u32 {
    let mut rows = Vec::new();
    for line in lines {
        rows.push(line.chars().into_iter().map(|c| c as u32 - '0' as u32).collect::<Vec<u32>>());
    }

    let mut count = 0;
    for row in 0..rows.len() {
        for column in 0..rows[row].len() {
            if !second_part {
                let mut up_blocked = false;
                let mut down_blocked = false;
                let mut right_blocked = false;
                let mut left_blocked = false;

                for up in 0..row {
                    up_blocked = up_blocked || rows[up][column] >= rows[row][column];
                }
                for down in (row+1)..rows.len() {
                    down_blocked = down_blocked || rows[down][column] >= rows[row][column];
                }
                for left in 0..column {
                    left_blocked = left_blocked || rows[row][left] >= rows[row][column];
                }
                for right in (column+1)..rows[row].len() {
                    right_blocked = right_blocked || rows[row][right] >= rows[row][column];
                }
                if !(up_blocked && down_blocked && left_blocked && right_blocked) {
                    count += 1;
                }
            } else {
                let mut up_distance = 0;
                let mut down_distance = 0;
                let mut right_distance = 0;
                let mut left_distance = 0;

                for up in (0..row).rev() {
                    up_distance += 1;
                    if rows[up][column] >= rows[row][column] {
                        break;
                    }
                }
                for down in (row+1)..rows.len() {
                    down_distance += 1;
                    if rows[down][column] >= rows[row][column] {
                        break;
                    }
                }
                for left in (0..column).rev() {
                    left_distance += 1;
                    if rows[row][left] >= rows[row][column] {
                        break;
                    }
                }
                for right in (column+1)..rows[row].len() {
                    right_distance += 1;
                    if rows[row][right] >= rows[row][column] {
                        break;
                    }
                }
                count = std::cmp::max(count, up_distance*down_distance*right_distance*left_distance);
            }
        }
    }
    count

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_days() {
        // Results are specific to the specific input files stored in the repo.
        assert_eq!(super::call_day_func(1, false),     "71934");
        assert_eq!(super::call_day_func(1, true),     "211447");
        assert_eq!(super::call_day_func(2, false),     "13268");
        assert_eq!(super::call_day_func(2, true),      "15508");
        assert_eq!(super::call_day_func(3, false),      "8109");
        assert_eq!(super::call_day_func(3, true),       "2738");
        assert_eq!(super::call_day_func(4, false),       "507");
        assert_eq!(super::call_day_func(4, true),        "897");
        assert_eq!(super::call_day_func(5, false), "TQRFCBSJJ");
        assert_eq!(super::call_day_func(5, true),  "RMHFJNVFP");
        assert_eq!(super::call_day_func(6, false),      "1134");
        assert_eq!(super::call_day_func(6, true),       "2263");
        assert_eq!(super::call_day_func(7, false),    "919137");
        assert_eq!(super::call_day_func(7, true),    "2877389");
        assert_eq!(super::call_day_func(8, false),      "1798");
        assert_eq!(super::call_day_func(8, true),     "259308");
    }
}
