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

    /// run the sample for the day.
    #[argh(switch, short='S')]
    sample: bool,
}

fn main() {
    let args:AdventArgs = argh::from_env();

    println!("day {}: {}", args.day_number, call_day_func(args.day_number, args.second_part, args.sample));
}

fn call_day_func (day_number:u8, second_part:bool, sample:bool) -> String {
    let inputfile = format!("day{}_{}input.txt", day_number, 
                            if sample {"sample_"} else {""});

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
            9 => {format!("{}", day9(lines, second_part))},
           10 => {format!("{}", day10(lines, second_part))},
           11 => {format!("{}", day11(lines, second_part))},
           12 => {format!("{}", day12(lines, second_part))},
           13 => {format!("{}", day13(lines, second_part))},
           14 => {format!("{}", day14(lines, second_part))},
           15 => {format!("{}", day15(lines, second_part, sample))},
           16 => {format!("{}", day16(lines, second_part))},
           17 => {format!("{}", day17(lines, second_part))},
           18 => {format!("{}", day18(lines, second_part))},
           19 => {format!("{}", day19(lines, second_part))},
           20 => {format!("{}", day20(lines, second_part))},
           21 => {format!("{}", day21(lines, second_part))},
           22 => {format!("{}", day22(lines, second_part, sample))},
           23 => {format!("{}", day23(lines, second_part))},
           24 => {format!("{}", day24(lines, second_part))},
           25 => {day25(lines, second_part)},
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
    calories.push(current);
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
                let height = rows[row][column];
                let fold_check = {|(blocked,current), h| (blocked || h >= &current, height)};
                let (up_blocked,_)    =  rows[0..row].iter().map(|c| &c[column]).fold((false,height), fold_check);
                let (down_blocked,_)  =  rows[row+1..rows.len()].iter().map(|c| &c[column]).fold((false,height), fold_check);
                let (left_blocked,_)  =  rows[row][0..column].iter().fold((false,height), fold_check);
                let (right_blocked,_) =  rows[row][column+1..rows[row].len()].iter().fold((false,height), fold_check);

                if !(up_blocked && down_blocked && left_blocked && right_blocked) {
                    count += 1;
                }
            } else {
                let mut up_distance = 0;
                let mut down_distance = 0;
                let mut right_distance = 0;
                let mut left_distance = 0;

                let count_func = |s:Vec<u32>, distance:&mut u32| { 
                    for t in s {
                        *distance += 1;
                        if t >= rows[row][column] { break; }
                    }};
                count_func(rows[0..row].iter().rev().map(|a| a[column]).collect::<Vec<u32>>(), &mut up_distance);
                count_func(rows[row+1..rows.len()].iter().map(|a| a[column]).collect::<Vec<u32>>(), &mut down_distance);
                count_func(rows[row][0..column].iter().rev().map(|a| *a).collect::<Vec<u32>>(), &mut left_distance);
                count_func(rows[row][column+1..rows[row].len()].iter().map(|a| *a).collect::<Vec<u32>>(), &mut right_distance);

                count = std::cmp::max(count, up_distance*down_distance*right_distance*left_distance);
            }
        }
    }
    count
}

fn day9(lines:Vec<String>, second_part:bool) -> u32 {
    let rope_length = if second_part {10} else {2};
    let mut rope_positions = vec![(0,0);rope_length];
    let mut tail_hash_set = HashSet::new();

    let tail_func = |head:(i32,i32), tail:(i32,i32) | {
        if {head.0 - tail.0}.abs() > 1 || {head.1 - tail.1}.abs() > 1 {
               ({head.0 - tail.0}.signum() + tail.0, tail.1 + {head.1 - tail.1}.signum())
           } else {
               tail
           }
    };

    tail_hash_set.insert(rope_positions[rope_positions.len()-1]);
    for line in lines {
        let move_split = line.split_whitespace().collect::<VecDeque<&str>>();
        let distance = move_split[1].parse::<u32>().unwrap();
        for _ in 0..distance {
            let movement = match move_split[0] {
                "R" => {( 1, 0)},
                "L" => {(-1, 0)},
                "U" => {( 0, 1)},
                "D" => {( 0,-1)},
                _ => {panic!("Unknown move {}:", move_split[0]);}
            };
            rope_positions[0] = {(rope_positions[0].0 + movement.0, 
                                  rope_positions[0].1 + movement.1)};

            for i in 0..(rope_positions.len()-1)  {
                rope_positions[i+1] = tail_func(rope_positions[i],
                                              rope_positions[i+1]);
            }
            tail_hash_set.insert(rope_positions[rope_positions.len()-1]);
        }
    }

    tail_hash_set.len() as u32
}

fn day10(lines:Vec<String>, second_part:bool) -> String {
    let (add, noop) = ("addx", "noop");
    let (add_cycles, noop_cycles) = (2, 1);
    let (special_offset, special_modulo) = (20, 40);
    let mut cycles = 0;
    let mut x = 1;
    let mut next_capture = special_offset;
    let mut signal_strength = 0;
    let mut display = "".to_string();
    for line in lines {
        let (x_offset, instruction_cycles) = match line {
            line if line.contains(add) => {
                let offset = line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap();
                (offset, add_cycles)}
            line if line.contains(noop) => {(0, noop_cycles)}, // Just have 'noop' change x by 0
            _ => {panic!("Unexpected instruction {}", line);}
        };

        for _i in 0..instruction_cycles {
            if second_part {
                if cycles % 40 == 0 {
                    display += "\n";
                }
                display += if {x-1..=x+1}.contains(&(cycles % 40)) {"#"} else {"."};
            }

            cycles += 1;
            if cycles == next_capture {
                signal_strength += next_capture * x;
                next_capture += special_modulo;
            }
        }
        x += x_offset;
    }

    if second_part {
        display
    } else {
        format!("{}", signal_strength)
    }
}

fn day11(lines:Vec<String>, second_part:bool) -> u64 {
    let (monkey_token, starting_items_token, operation_token, test_token, true_token, false_token) = 
        ("Monkey ", "  Starting items: ", "  Operation: new = ", 
         "  Test: divisible by ", "    If true: throw to monkey ", "    If false: throw to monkey ");
    // Operations: 'new = A * B', A-> num or old, A-> num or old.
    
    struct Monkey {
        id: u32,
        items: Vec<u64>,
        operation: Box<dyn Fn(u64) -> u64>,
        test:      u64,
        true_monkey: u32,
        false_monkey: u32,
    }

    let mut monkey;
    let mut test;
    let mut true_monkey;
    let mut false_monkey;
    let mut monkeys = Vec::<Monkey>::new();

    let mut iter = lines.into_iter();
    while let Some(mut line) = iter.next() {

        if "" == line { 
            line = iter.next().unwrap();
        }
        monkey = line[monkey_token.len()..(line.len()-1)].parse::<u32>().unwrap();
        line = iter.next().unwrap();
        let items = line[starting_items_token.len()..].split(", ").map(|l| l.parse::<u64>().unwrap()).collect::<Vec<u64>>();

        line = iter.next().unwrap();
        let operation:Box::<dyn Fn(u64) -> u64> = match line {
            line if line.contains("*") && 2 == line.matches("old").count() => {Box::new(|x| {x*x})},
            line if line.contains("+") && 2 == line.matches("old").count() => {Box::new(|x| {x+x})},
            line if line.contains("+") && 1 == line.matches("old").count() => {
                let operand = line[operation_token.len()..line.len()].split(" + ").nth(1).unwrap().parse::<u64>().unwrap();
                Box::new(move |x| {x+operand})
            },
            line if line.contains("*") && 1 == line.matches("old").count() => {
                let operand = line[operation_token.len()..line.len()].split(" * ").nth(1).unwrap().parse::<u64>().unwrap();
                Box::new(move |x| {x*operand})
            }
            _ => {panic!("Unexpected match.");}
        };

        line = iter.next().unwrap();
        test = line[test_token.len()..line.len()].parse::<u64>().unwrap();

        line = iter.next().unwrap();
        true_monkey = line[true_token.len()..line.len()].parse::<u32>().unwrap();

        line = iter.next().unwrap();
        false_monkey = line[false_token.len()..line.len()].parse::<u32>().unwrap();

        assert_eq!(monkey as usize, monkeys.len());  // Check that they are sequential.
        monkeys.push(Monkey{id:monkey, items:items, operation: operation, test: test, true_monkey:true_monkey, false_monkey:false_monkey});
    }


    // Get the items, so they're mutable.
    let mut divisor = 1;
    let mut monkey_items = Vec::<VecDeque::<u64>>::new();
    let mut inspections = vec![0;monkeys.len()];
    for current_monkey in monkeys.iter() {
        divisor *= current_monkey.test;
        monkey_items.push(VecDeque::<u64>::new());
        for i in current_monkey.items.iter() {
            monkey_items[current_monkey.id as usize].push_back(*i);
        }
    }

    for _round in 0.. if !second_part{20} else {10000} {
        for current_monkey in monkeys.iter() {
            for _ in 0..monkey_items[current_monkey.id as usize].len() {
                inspections[current_monkey.id as usize] += 1;
                let mut worry = monkey_items[current_monkey.id as usize].pop_front().unwrap();
                worry = (current_monkey.operation)(worry);
                worry /= if !second_part{3} else {1};
                if 0 == worry % current_monkey.test {
                    monkey_items[current_monkey.true_monkey as usize].push_back(worry % divisor);
                } else {
                    monkey_items[current_monkey.false_monkey as usize].push_back(worry % divisor);
                }
            }
        }
    }
    inspections.sort();
    inspections.reverse();
    inspections[0..2].iter().fold(1,|total,i| {total * i})
}

fn day12(lines:Vec<String>, second_part:bool) -> u32 {
    let char_to_height = |x| {
        match x {
            'a'..='z' => {x as u32 - 'a' as u32},
            'S' => {'a' as u32 - 'a' as u32},
            'E' => {'z' as u32 - 'a' as u32},
            _ => {panic!("Unexpected char {}", x);}
        }
    };

    let mut grid = Vec::<Vec::<u32>>::new();
    let mut start = (0,0);
    let mut destination = (0,0);

    for (index, line) in lines.iter().enumerate() {
        grid.push(Vec::<u32>::new());
        grid[index] = line.chars().into_iter().map(|c| char_to_height(c)).collect();
        if line.contains('S') {start = (line.chars().position(|c| c == 'S').unwrap(),index)};
        if line.contains('E') {destination = (line.chars().position(|c| c == 'E').unwrap(),index)};
    }

    let width = grid[0].len();
    let height = grid.len();
    let max_distance = width * height;
    let mut distance = vec![vec![max_distance;width];height];

    let update_distance = |grid:&Vec::<Vec::<u32>>, distance:&mut Vec::<Vec::<usize>>, location:(usize,usize), i| {
        if distance[location.1][location.0] == i {
            // check up, down, left, right
            let offsets:Vec::<(i32,i32)> = vec![(1,0), (-1,0), (0,1), (0,-1)];
            for offset in offsets {
                if (location.0 as i32 + offset.0) >= 0 && (location.0 as i32 + offset.0) < width as i32 && 
                   (location.1 as i32 + offset.1) >= 0 && (location.1 as i32 + offset.1) < height as i32 && 
                    distance[(location.1 as i32 + offset.1) as usize][(location.0 as i32 + offset.0) as usize] == max_distance &&
                    ((grid[(location.1 as i32 + offset.1) as usize][(location.0 as i32 + offset.0) as usize] + 1) >= (grid[location.1][location.0]))
                    {
                        distance[(location.1 as i32 + offset.1) as usize][(location.0  as i32+ offset.0) as usize] = distance[location.1][location.0] + 1;
                    }
            }
        }
    };

    distance[destination.1][destination.0] = 0;

    // Stop once the start has been reached
    let mut i = 0;
    while max_distance == distance[start.1][start.0] {
        let mut original_distance = distance;
        for y in 0..height {
            for x in 0..width {
                update_distance(&grid, &mut original_distance, (x,y), i);
            }
        }
        i += 1;
        distance = original_distance;
    }

    // Find the 'a' closest to the destination
    let mut closest = max_distance;
    if second_part {
        for y in 0..height {
            for x in 0..width {
                if 0 == grid[y][x] {
                    closest = std::cmp::min(distance[y][x], closest);
                }
            }
        }
    } else {
        closest = distance[start.1][start.0];
    }

    closest as u32 - distance[destination.1][destination.0] as u32
}

//fn blah(lines:<Vec<String> as Trait>::Iter, second_part:bool) -> u32 {
//    0
//}

fn day13(lines:Vec<String>, second_part:bool) -> u32 {

    fn compare_list(left_string:String, right_string:String) -> bool {
        let left_input = left_string.chars().collect::<Vec::<char>>();
        let right_input = right_string.chars().collect::<Vec::<char>>();

        let get_first_digit = |input:&[char]| {input.iter().take_while(|c| c.is_digit(10)).collect::<String>().parse::<u32>().unwrap()};
        let skip_first_digit = |input:&[char]| {input.iter().skip_while(|c| c.is_digit(10)).collect::<String>()};

        for ((l_idx, l), (r_idx, r)) in std::iter::zip(left_input.iter().enumerate(), right_input.iter().enumerate()) {
            match (l, r) {
                ('[','[') => {},
                (',',',') => {},
                (i,j) if i.is_digit(10) && j.is_digit(10) => {
                    let left_num = get_first_digit(&left_input[l_idx..]);
                    let right_num = get_first_digit(&right_input[l_idx..]);
                    if left_num < right_num {
                        return true;
                    } else if left_num > right_num {
                        return false;
                    }
                    else {
                        return compare_list(left_input[l_idx..].iter().skip_while(|c| c.is_digit(10)).collect(), 
                                            right_input[l_idx..].iter().skip_while(|c| c.is_digit(10)).collect());
                    }
                },
                // If left is not a list, only need to compare the first value
                (i,'[') if i.is_digit(10) => {
                    let left_num = get_first_digit(&left_input[l_idx..]);
                    return compare_list(format!("{}]{}",left_num, skip_first_digit(&left_input[(l_idx+1)..])),
                                         right_input[(r_idx+1)..].iter().collect());

                },
                // If right is not a list, only need to compare the first value
                ('[',j) if j.is_digit(10) => {
                    let right_num = get_first_digit(&right_input[l_idx..]);
                    return compare_list(left_input[(l_idx+1)..].iter().collect(),
                                        format!("{}]{}",right_num, skip_first_digit(&right_input[(r_idx+1)..])));
                },
                (']',']') => {},
                (']',_) => {return true}, // Left finished early
                (_,']') => {return false},
                _ => {panic!("Unmatched case {} {}", l, r);}
            }
        }
        true
    }

    let mut remove_gaps = lines.clone().iter().filter(|x| **x != "".to_string()).map(|l| l.to_string()).collect::<Vec<String>>();
    if !second_part {
        let mut iter = remove_gaps.into_iter();
        let mut sum_in_order = 0;
        let mut index = 1;
        while let Some(first_packet) = iter.next() {
            let second_packet = iter.next().unwrap();
            if compare_list(first_packet, second_packet) {
                sum_in_order += index;
            }
            index += 1;
        }
        sum_in_order
    } else {
        let first_marker = "[[2]]".to_string();
        let second_marker = "[[6]]".to_string();
        remove_gaps.push(first_marker.clone()); 
        remove_gaps.push(second_marker.clone()); 
        remove_gaps.sort_by(|a,b| {if compare_list(a.to_string(),b.to_string()) {std::cmp::Ordering::Less} else {std::cmp::Ordering::Greater}});

        // 1-based
        let first_dividor = remove_gaps.iter().position(|x| *x == first_marker).unwrap() + 1;
        let second_dividor = remove_gaps.iter().position(|x| *x == second_marker).unwrap() + 1;

        (first_dividor * second_dividor) as u32
    }
}

fn day14(lines:Vec<String>, second_part:bool) -> u32  {
    // Read lines of rock (x,y) -> (x,y)
    // x = distance to right, y = distance down
    // sand = '+', 1 unit at a time
    // sand moves 'down', then diagonally 'down left'-> 'down right' keeps moving until blocked.
    // Stop when sand falls greater than lowest rock.
    
    // Just represent the cave as a 'hash set' of points, rather than a grid. Once settled, sand
    // behaves like rock.
    let mut cave = HashSet::<(u32,u32)>::new();

    for line in lines {
        // Convert the line to a list of coordinates.
        let coordinates = line.split(" -> ").map(|point| {let p = point.split(",").map(|value| {value.parse::<u32>().unwrap()}).collect::<Vec<u32>>();(p[0], p[1])}).collect::<Vec<(u32,u32)>>();

        let mut coord_it = coordinates.iter().peekable(); 
        while let Some(coord) = coord_it.next() {
            if let Some(next) = coord_it.peek()
            {
                let either_order_range = |a,b| {std::cmp::min(a,b)..=std::cmp::max(a,b)};
                for x in either_order_range(coord.0, next.0) {
                    for y in either_order_range(coord.1, next.1) {
                        cave.insert((x,y));
                    }
                }
            }
        }
    }

    let mut max_depth = cave.iter().fold(0, |s, d| {std::cmp::max(s, d.1)});

    if second_part { 
        max_depth += 1;
    }

    let mut grains = 0;
    let mut sand_position = (500, 0);
    let mut keep_going = true;
    while keep_going {

        let offsets:Vec::<(i32,i32)> = vec![(0,1), (-1,1), (1,1)];
        let apply_offset = |point:(u32,u32), offset:(i32,i32)| {((point.0 as i32 + offset.0) as u32, 
                                                                 (point.1 as i32 + offset.1) as u32)};
        let mut new_grain = false;
        if let Some(empty_offset) = offsets.iter().find(|v| {!cave.contains(&apply_offset(sand_position, **v))}) {
            sand_position = apply_offset(sand_position, *empty_offset);
            
            if second_part {
                // Create a new grain if it hit the max depth allowed.
                new_grain = sand_position.1 == max_depth;
            }
        } else {
            new_grain = true;
        }

        if sand_position == (500,0) {
            // Second part, if the end point is the start, then stop
            keep_going = false;
        }

        if new_grain {
            // If the sand 'hits the bottom'
            grains += 1; // Count the grains of sand.
            cave.insert(sand_position);
            sand_position = (500, 0);
        }

        if sand_position.1 == max_depth {
            keep_going = false;
        }
    }

    grains
}

fn day15(lines:Vec<String>, second_part:bool, sample:bool) -> u64  {

    let mut info = Vec::<(i32,i32,i32,i32)>::new();
    let mut beacon_positions = HashSet::<(i32,i32)>::new();
    let sensor_beacon_distance:Vec::<(i32,i32,u32)>;
    // Sensor at x=193758, y=2220950: closest beacon is at x=652350, y=2000000
    for line in lines {
        let line_iter = line.chars().into_iter();
        let numbers = line_iter.filter(|x| x.is_digit(10) || *x=='-' || *x == ' ')
                           .collect::<String>()
                           .split_whitespace()
                           .map(|s| s.parse::<i32>().unwrap())
                           .collect::<Vec<i32>>();
        // Sensor, beacon
        info.push((numbers[0],numbers[1],numbers[2],numbers[3]));
        beacon_positions.insert((numbers[2],numbers[3]));
    }
    sensor_beacon_distance = info.iter().map(|(sx,sy,bx,by)| {
                                (*sx,*sy, ((sx-bx).abs() + (sy-by).abs()) as u32) })
                                  .collect();
    let no_beacon = |sensor_distance:(i32,i32,u32), position:(i32,i32)| {(((sensor_distance.0-position.0).abs() + 
                                                   (sensor_distance.1-position.1).abs()) as u32) <=  sensor_distance.2};


    let mut result:u64 = 0; 
    if !second_part {
        let minx = sensor_beacon_distance.iter().fold(0,|s, (sx,_sy,distance)| std::cmp::min(s, *sx - (*distance as i32)));
        let maxx = sensor_beacon_distance.iter().fold(0,|s, (sx,_sy,distance)| std::cmp::max(s, *sx + (*distance as i32)));

        for x in minx..=maxx {
            let y = if !sample{2000000} else {10}; // Search row for sample input is '10'
            let test_position = (x, y);
            if sensor_beacon_distance.iter().any(|sbd| {no_beacon(*sbd, test_position)})
                && !beacon_positions.contains(&test_position) {
                    // Increase if within the 'no beacon' zone and it's not a beacon position.
                    result += 1;
                }
        }
    } else {
        // Test all points at a distance+1 from the closest beacon of each sensor.
        let mut test_positions = Vec::<(i32,i32)>::new();
        
        for (sx, sy, distance) in &sensor_beacon_distance {
            let check_distance = (distance + 1) as i32;
            for i in 0..check_distance {
                test_positions.push((sx + i, sy + (check_distance - i)));
                test_positions.push((sx - i, sy - (check_distance - i)));
                test_positions.push((sx + (check_distance - i), sy + i));
                test_positions.push((sx - (check_distance - i), sy - i));
            }
        }

        let max_search_area = if !sample{4000000} else {20}; // Search area for sample input is '20'
        for test_position in test_positions.iter().filter(|(x,y)| {0..=max_search_area}.contains(x) && {0..=max_search_area}.contains(y) ) {
            if sensor_beacon_distance.iter().all(|sbd| {!no_beacon(*sbd, *test_position)})
                && !beacon_positions.contains(&test_position) {
                    // Increase if within the 'no beacon' zone and it's not a beacon position.
                    result = test_position.0 as u64 * 4000000 + test_position.1 as u64;
                    break; // Expect 1 result, so break out of the loop
            }
        }
    }

    result
}

fn day16(lines:Vec<String>, second_part:bool) -> u32  {
    // Get the shortest distance between each pair of Valves with a non-zero flow rate.
    // Never turn on a valve with a zero flow rate.
    // Keep a set of valves that have been turned on
    // Keep a set of valves that are off (non-zero flow rate).

    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    
    let mut valve_flow_rates:HashMap<String,u32> = HashMap::new();
    let mut tunnel_links = HashMap::new();
    let mut unopen_valves = HashSet::new();
    for line in lines {
        let no_commas = line.replace([',',';'],"");
        let words = no_commas.split_whitespace().collect::<Vec<&str>>();
        let valve = words[1].to_string();
        for word in &words {
            match *word {
                "Valve" => {},
                word if word.contains("rate=") => {
                    valve_flow_rates.insert(valve.clone(), word.strip_prefix("rate=").unwrap().parse::<u32>().unwrap());
                },
                _ => {}

            }
        }
        let exit_tunnels = words.iter().rev().cloned()
                                .take_while(|w| !w.contains("valve"))
                                .map(|w| w.to_string())
                                .collect::<Vec<String>>();
          tunnel_links.insert(valve.clone(), exit_tunnels.iter().cloned().collect::<Vec<String>>()); 
    }

    for (valve, rate) in &valve_flow_rates {
        if *rate > 0 {
            unopen_valves.insert(valve.to_string());
        }
    }

    fn shortest(tunnel_links: &HashMap<String, Vec::<String>>, src:&String, dst:&String) -> u32  {
        let mut next_locations = Vec::<(String, u32)>::new();
        next_locations.push((src.to_string(), 0));
        loop {
            if let Some(result) = next_locations.iter().find(|(s,_d)| s == dst) {
                return result.1;
            }

            next_locations = next_locations.iter().flat_map(|(s1,d1)| 
                                                       tunnel_links.get(s1)
                                                                   .unwrap().iter().map(|d2| {(d2.to_string(), *d1+1)}))
                                                                   .collect::<Vec<(String, u32)>>();
        }
    }

    let mut shortest_paths:HashMap<(String,String),u32>  = HashMap::new();
    for src in &unopen_valves {
        for dst in &unopen_valves {
            let distance = shortest(&tunnel_links, &src, &dst);
            shortest_paths.insert((src.to_string(), dst.to_string()), distance);
        }
    }

    let starting_point = "AA".to_string();
    for dst in &unopen_valves {
        shortest_paths.insert((starting_point.to_string(), dst.to_string()), 
                              shortest(&tunnel_links, &starting_point, &dst));
    }

    // Visit as many valves as possible.
    // For each unvisited valve:
    //     - Move to the valve
    //     - Open the valve, update the 'final' value based on the time remaining when the valve is
    //     opened
    //
    //     Return the total.

    fn total_flow(rates:&HashMap<String, u32>, tunnel_links: &HashMap<String, Vec::<String>>, 
                  shortest: &HashMap<(String,String),u32>, 
                  save_result: &mut Vec::<(u32, HashSet::<String>)>, 
                  location:String, remaining_valves: &HashSet::<String>, time_remaining:u32, current_total:u32, watermark:u32) -> (u32,HashSet::<String>)  {
        // If 'watermark' is set to 0, don't save.  Otherwise save any result that's larger than
        // the mark (to use for subsequent searches).
        if 0 != watermark && current_total >= watermark {save_result.push((current_total, remaining_valves.clone()));}

        if 0 == time_remaining {
            return (current_total,remaining_valves.clone());
        }
        if remaining_valves.len() == 0 {
            (current_total, remaining_valves.clone())
        } else {
              let remaining_rates = rates.iter().filter(|(k,_d)| remaining_valves.contains(*k)).collect::<Vec<(&String,&u32)>>();

              remaining_rates.iter().fold((0, remaining_valves.clone()),  |max, (d,rate) | {
                let path = {
                    let distance = shortest.get(&(location.clone(), d.to_string())).unwrap();
                    if time_remaining > *distance {
                        let mut after_remove = remaining_valves.clone();
                        after_remove.remove(*d);
                        let new_accumulation = *rate * (time_remaining - distance - 1);
                        total_flow(rates, tunnel_links, shortest, save_result, d.to_string(),
                                            &after_remove, time_remaining - distance - 1, current_total + new_accumulation, watermark)
                    } else {
                        (current_total, remaining_valves.clone())
                    }
                };
                if max.0 > path.0 {
                    (max.0, max.1)
                } else {
                    path
                }
            }
            )
        }
    }

    let mut save_result = Vec::new();
    let max_time = if !second_part {30} else {26};

    // Find the initial 'best' result for the given time
    let flow_result = total_flow(&valve_flow_rates, &tunnel_links, &shortest_paths, &mut save_result,
                                    starting_point.to_string(), &unopen_valves, max_time, 0, 0);

    if second_part {
        save_result = Vec::new();
        // Using the 'left over' valves, find the best 'elephant' result (this is the 'minimum'
        // that the second helper should be able to cover).
        let elephant_flow_result = total_flow(&valve_flow_rates, &tunnel_links, &shortest_paths, &mut save_result,
                                              starting_point.to_string(), &flow_result.1, max_time, 0, 0);

        // Now re-run to find all results at least as good as the second run.
        save_result = Vec::new();
        let _ = total_flow(&valve_flow_rates, &tunnel_links, &shortest_paths, &mut save_result,
                           starting_point.to_string(), &unopen_valves, max_time, 0, std::cmp::max(1, elephant_flow_result.0));

        let mut total_max = 0;
        let mut subset = HashMap::new();

        // Get a unique set of unopened valves, picking the 'best' if there are duplicates.
        // This could be done as part of the 'save'.
        for (total_for_save, search_set) in &save_result {
            let mut sorted_set = search_set.iter().cloned().collect::<Vec<String>>();
            sorted_set.sort(); 
            let current_size = *subset.entry(sorted_set.clone()).or_insert(total_for_save);
            if current_size < total_for_save {
                *subset.get_mut(&sorted_set).unwrap() = total_for_save;
            }
        }

        for (search_set, total_for_save) in subset {
            // For each remainder, find the best total.
            let other_result = total_flow(&valve_flow_rates, &tunnel_links, &shortest_paths, &mut Vec::new(),
                                         starting_point.to_string(), 
                                         &search_set.iter().cloned().collect::<HashSet::<String>>(),
                                         max_time, *total_for_save, 0);
            total_max = std::cmp::max(total_max, other_result.0);
        }

        total_max 
    } else {
        flow_result.0
    }
}

fn day17(lines:Vec<String>, second_part:bool) -> u64  {

    // They start with 2 spaces on the left.
    // ####
    // 
    // .#.
    // ###
    // .#.
    // 
    // ..#
    // ..#
    // ###
    // 
    // #
    // #
    // #
    // #
    // 
    // ##
    // ##

    // Get tetris type pieces

    let blocks = vec![vec![vec![1,1,1,1]],

                      vec![vec![0,1,0],
                           vec![1,1,1],
                           vec![0,1,0]],

                      vec![vec![0,0,1],
                           vec![0,0,1],
                           vec![1,1,1]],

                      vec![vec![1],
                           vec![1],
                           vec![1],
                           vec![1]],

                      vec![vec![1,1],
                           vec![1,1]]];



    fn print_game_state(game_state:&Vec<Vec<i32>>) {
        for line in game_state.iter().rev() {
            println!("{}", line.iter().fold("".to_string(), |sum, i| format!("{}{}", sum, if *i==0 {'.'} else {'#'})));
        }
        println!("");
    }
    // Returns 'true' if the block for the given height/offset overlaps. False otherwise.
    fn check_intersection(game_state:&Vec<Vec<i32>>, gap_width:usize, block:&Vec<Vec<i32>>, block_height:i32, block_offset:i32) -> bool
    {
        let mut intersects = false;
        for y in 0..block.len() {
            
            // Check edges and bottom.
            if block_offset < 0 || (block_offset + block[0].len() as i32) as usize > gap_width || (block_height + y as i32) < 0 {
                intersects = true;
            } else {
                let board_index = (y as i32 + block_height) as usize;
                if board_index < game_state.len() {
                    for x in 0..block[block.len() - y - 1].len() {
                        if x as i32 + block_offset < game_state[board_index].len() as i32 {
                            if board_index < game_state.len() {
                                if 1 == block[block.len() - y - 1][x as usize] && 1 == game_state[board_index][x + block_offset as usize] {
                                    intersects = true;
                                }
                            }
                        } else {
                            panic!("Need to check this condition.");
                        }
                    }
                }
            }
        }
        intersects
    }

    #[derive(PartialEq, Clone, Copy)]
    enum State {
        NewBlock,
        Jet,
        MoveDown,
        Stopped,
    }
    let mut current_game_state = State::NewBlock;

    let gap_width:usize = 7;
    let mut game_state = Vec::<Vec::<i32>>::new();

    let mut block_height_pairs = Vec::<(usize, usize)>::new();

    for line in lines {
        // Start with a 'floor'
        let mut current_block = blocks[0].clone();
        let mut x_offset:i32 = 2; 
        let mut movement = line.chars().cycle(); // Characters should go forever

        let blocks_to_test = if second_part {10000} else {2022};        
        for current_block_number in 0..blocks_to_test {
            let mut block_height = game_state.len() + 3;

            while current_game_state != State::Stopped {
                match current_game_state { 
                    State::NewBlock => {current_block = blocks[current_block_number % blocks.len()].clone();
                        x_offset = 2; // Blocks start 2 to the right. 
                        current_game_state = State::Jet;
                    },
                    State::Jet => {
                        let next_move = movement.next().unwrap();
                        let next_offset = match next_move { '>' => {1}, '<' => {-1}, _ => {panic!("blah");}};
                        if false == check_intersection(&game_state, gap_width, &current_block, block_height as i32, x_offset + next_offset) {
                            x_offset += next_offset;
                        }
                        current_game_state = State::MoveDown;
                    },
                    State::MoveDown => {
                        if check_intersection(&game_state, gap_width, &current_block, block_height as i32 - 1, x_offset) {
                            current_game_state = State::Stopped;
                        } else {
                            block_height -= 1;
                            current_game_state = State::Jet;
                        }
                    },
                    State::Stopped => {()},
                }
            }

            // Store the block
            for y in 0..current_block.len() {
                let game_height = y + block_height as usize;
                if game_height >= game_state.len() {
                    game_state.insert(game_height, vec![0;gap_width]);
                }

                for x in 0..current_block[current_block.len() - y - 1].len() {
                    game_state[game_height][x + x_offset as usize] |= current_block[current_block.len() - y - 1][x];
                }
            }
            block_height_pairs.push((current_block_number, game_state.len())); 
            current_game_state = State::NewBlock;
        }
    }


    if second_part {
        // Find a location where the sequence of height changes repeats.
        // Get an arbitrary string (long enough to have confidence that it's unique)
        // Find a subsequent location of the string.
        // Find offsets that line up with the 'final block number', query for the hight at that
        // point.
        // Use the calculated deltas to determine what the result would be at the final block.
        let mut sequence = Vec::<usize>::new();
        let mut last = 0;
        for block_height in &block_height_pairs {
            sequence.push(block_height.1 - last);
            last = block_height.1;
        }

        let search_offset = 443;
        let search_size = 500; // Arbitrary, just needs to be smaller than the repetition, but big enough for confidence.
        let search_splice = sequence[search_offset..(search_offset+search_size)].to_vec();

        let mut next_match = search_offset + 1;
        next_match =  next_match + sequence[next_match..].windows(search_splice.len()).position(|window| window == search_splice).unwrap();

        let last_block_number:u64 = 1000000000000;
        let sequence_repetition_length = (next_match - search_offset) as u64;
        let starting_point = last_block_number as u64 % sequence_repetition_length as u64;

        // 'block 1' is at index '0'
        let base_height = *&block_height_pairs[starting_point as usize - 1].1 as u64;
        let repetition_height_increment = (&block_height_pairs[(starting_point + 2 * sequence_repetition_length)  as usize - 1].1 - 
                                           &block_height_pairs[(starting_point + sequence_repetition_length)  as usize - 1].1) as u64;

        // Manual runs:
        // 318 @ 200
        // 2941 @ 1900
        // 5564 @ 3600 
        // 1542941176480
        println!("{} {} {} {}", last_block_number, sequence_repetition_length, repetition_height_increment , base_height);
        (((last_block_number as u64 / sequence_repetition_length as u64) * repetition_height_increment) + base_height) as u64

    } else {
        game_state.len() as u64
    }
}

fn day18(lines:Vec<String>, second_part:bool) -> u64  {
    let mut split_ints = Vec::new();
    for line in lines {
        split_ints.push(line.split(",").map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    }
    let mut lava_points = HashSet::new();
    let (mut maxx,mut maxy,mut maxz) = (0,0,0);
    for position in &split_ints {
        // Offsetting the points, so there are no zeros
        let new_point = (position[0]+1, position[1]+1, position[2]+1);
        maxx = std::cmp::max(maxx, new_point.0+1);
        maxy = std::cmp::max(maxy, new_point.1+1);
        maxz = std::cmp::max(maxz, new_point.2+1);
        lava_points.insert(new_point);
    }
    let touching = |a:&(i32,i32,i32),b:&(i32,i32,i32)| { if (a.0 == b.0 && a.1 == b.1 && (a.2 - b.2).abs() == 1) ||
        (a.0 == b.0 && (a.1 - b.1).abs() == 1 && a.2 == b.2) ||
            ((a.0 - b.0).abs() == 1 && a.1 == b.1 && a.2 == b.2) 
            { true } else { false }};

    if !second_part {
        let mut total_sides = 6 * split_ints.len();
        for cube in &lava_points {
            total_sides -= &lava_points.iter().filter(|x| touching(*x,cube)).count();
        }
        total_sides as u64
    } else {
        // Input, next set of 'air' points to check.
        // Output, new set of 'air' points and number of touching lava cubes.
        // Finish when there are no new 'air' points.
        fn increment_flood_fill(flooded: &mut HashSet::<(i32,i32,i32)>, 
                                lava_points: &HashSet::<(i32,i32,i32)>, 
                                search_points: &HashSet::<(i32,i32,i32)>,
                                max:(i32,i32,i32)) -> (HashSet::<(i32,i32,i32)>, u32) {
            let mut new_search_points = HashSet::<(i32,i32,i32)>::new();
            let offsets:Vec::<(i32,i32,i32)> = vec![(0,0,1), ( 0, 0,-1), 
                                                    (0,1,0), ( 0,-1, 0),
                                                    (1,0,0), (-1, 0, 0)];

            let apply_offset = |point:(i32,i32,i32), offset:(i32,i32,i32)| {((point.0 + offset.0), 
                                                                             (point.1 + offset.1),
                                                                             (point.2 + offset.2))};

            let mut lava_surfaces = 0;
            for look in search_points.iter() {
                for offset in offsets.iter() {
                    let check_point = apply_offset(*look, *offset);
                    if lava_points.contains(&check_point) {
                        lava_surfaces += 1;
                    } else if flooded.contains(&check_point) {
                        // Don't do anything.
                    } else {
                        if check_point.0 <= max.0 && check_point.1 <= max.1 && check_point.2 <= max.2
                           && check_point.0 >= 0 && check_point.1 >= 0 && check_point.2 >= 0
                        {
                            new_search_points.insert(check_point);
                        }
                    }
                }
                // Add the point to 'flooded' so it isn't checked again.
                flooded.insert(*look);
            }
            (new_search_points, lava_surfaces)
        }

        let mut flooded = HashSet::new();
        let mut search_points = HashSet::new();
        let mut last_lava_touch_count;
        let mut total_lava_surface = 0;
        flooded.insert((0,0,0));
        search_points.insert((0,0,0));

        while search_points.len() != 0 {
            (search_points, last_lava_touch_count) = increment_flood_fill(&mut flooded, &lava_points, &search_points, (maxx, maxy, maxz));
            total_lava_surface += last_lava_touch_count;
        }

        total_lava_surface as u64
    }
}

fn day19(lines:Vec<String>, second_part:bool) -> u32  {
    // Pick a blueprint
    // ore, clay, obsidian
    // Number of opened geodes after 24 minutes 
    //
    // 1 ore collecting robuts
    //
    // each robot can colect 1 of its resource type per minut
    // 1 minute for each robot of it's time to collect a resource 
    // 1 minute for to construct any robot, and consumes resource at start of construction
    //
    // Can only use 1 blueprint 
    //
    // Options:  
    //   Do Nothing
    //   build ore robot
    //   build clay robot
    //   build obsidian robot
    //   build geod robot
    //
    //   Resources, ore, clay, obsidian, geod, time
    
    let mut all_blueprints = Vec::new();
    for (_index, line) in lines.iter().enumerate() {
        // Assume 1 blueprint per line
        let numbers = line.chars()
                              .filter(|x| !(x.is_alphabetic() || *x == '.' || *x == ':'))
                              .collect::<String>()
                              .split_whitespace()
                              .map(|s| s.parse::<u32>().unwrap())
                              .collect::<Vec<u32>>();
        assert_eq!(numbers.len(), 7); 
        all_blueprints.push(vec![vec![numbers[1], 0, 0],
                             vec![numbers[2], 0, 0],
                             vec![numbers[3], numbers[4], 0],
                             vec![numbers[5], 0, numbers[6]]]);
    }

    // Find the shortest time to make x geod robots, if time is < 24, then that's the best
    fn find_max_geods(blueprints:&Vec<Vec<u32>>, active_robots:&Vec<u32>, dont_build:&Vec<bool>, resources:&Vec<u32>, time_left:u32, current_max:u32) -> u32 {
   
        #[derive(PartialEq, Clone, Copy)]
        enum Robot { /*Ore = 0, Clay = 1, Obsidian = 2,*/ Geode = 3}

        // Can build the robot if the resources are at lest the requirement for the blueprint
        let can_build_robot = |blueprint, resource| {std::iter::zip(blueprint, resource).fold(true, |s,(b,r)| s && r >= b)};

        if 0 == time_left {
            // Return the resources for the geod robot.
            resources[Robot::Geode as usize]
        } else {
            // Get the newly mined resources from previous robots.
            let new_resources = std::iter::zip(active_robots, resources).map(|(a,b)| a+b).collect::<Vec<u32>>();

            let mut max = current_max;

            // Get the maximum of each option that exists.
            // Assumes only 1 robot can be built per time step.
            for (i, current_blueprint) in blueprints.iter().enumerate().rev() {
                if i < Robot::Geode as usize && 
                   blueprints.iter().fold(0, |m,b| std::cmp::max(m,b[i])) <= active_robots[i] {
                    // Don't build too many robots of the same type.
                } else if can_build_robot(current_blueprint, resources) && dont_build[i] == false 
                    // Don't keep searching if it's not possible for this path to reach the maximum
                    && current_max < resources[Robot::Geode as usize] + (time_left) * (active_robots[Robot::Geode as usize]) + ((time_left) * (time_left)/2) 
                    { 
                    // Use up the resource to build the robot.
                    let mut new_robots = active_robots.clone();
                    new_robots[i] += 1;

                    // Reduce the 'new' resources
                    let mut reduced_resources = std::iter::zip(&new_resources, current_blueprint).map(|(a,b)| a-b).collect::<Vec<u32>>();
                    reduced_resources.push(new_resources[Robot::Geode as usize]); // Geod isn't in the blueprint, so carry over. 

                    max = std::cmp::max(max, find_max_geods(blueprints, &new_robots, &vec![false;4], &reduced_resources, time_left-1, max));
                }
            }

            // Start with max if no robots are built.
            let build_options =  blueprints.iter().map(|b| can_build_robot(b, resources)).collect::<Vec<bool>>();
        
            std::cmp::max(max, find_max_geods(blueprints, &active_robots, &build_options, &new_resources, time_left-1, max))
        }
    }

    let mut quality = 0;
    let mut result = 1;
    for (index, blueprints) in all_blueprints.iter().enumerate() {
        let robots    = vec![1, 0, 0, 0]; // Always start with 1 ore robot.
        let resources = vec![0, 0, 0, 0];

        if !second_part {
            // Goal, most geodes in 24 minutes
            let geodes = find_max_geods(&blueprints, &robots, &vec![false;4], &resources, 24, 0);
            quality += geodes * (index as u32 + 1);
        } else {
            if index <= 2 {
                let geodes = find_max_geods(&blueprints, &robots, &vec![false;4], &resources, 32, 0);
                result *= geodes;
            }
        }
    }

    if !second_part { quality } else { result }
}

fn day20(lines:Vec<String>, second_part:bool) -> i64  {
    let mut code = Vec::new();
    let (multiplier, num_cycles) = if second_part {(811589153,10)} else {(1,1)};

    for (i, line) in lines.iter().enumerate() {
        code.push((i, line.parse::<i64>().unwrap() * multiplier));
    }

    let mut new_code = Vec::new();
    for _cycle in 0..num_cycles {
        new_code = code.clone();
        for i in 0..code.len() {
            let current_position = new_code.iter().position(|(original_index,_v)| *original_index == i).unwrap(); 
            let (original_index, value) = new_code[current_position];
            new_code.remove(current_position);
            let new_position = ((((current_position as i64 + value) as i64) % new_code.len() as i64) + new_code.len() as i64) % new_code.len() as i64;
            new_code.insert(new_position as usize, (original_index, value));
        }
        code = new_code.clone();
    }

    let mut result:i64 = 0;
    let zero_pos = new_code.iter().position(|(_original_index,v)| *v == 0).unwrap(); 
    for search_index in vec![1000, 2000, 3000].iter() {
        result += new_code[(zero_pos + search_index) % new_code.len()].1;
    }
    result as i64
}

fn day21(lines:Vec<String>, second_part:bool) -> i64  {

    let mut lookup = HashMap::new();
    for line in lines.iter() {
        let elements = line.split(": ").collect::<Vec<&str>>();
        lookup.insert(elements[0], elements[1]);
    }

    fn resolve_values(lookup:&HashMap::<&str,&str>, current_node:&str) -> i64 {
        let value = lookup.get(current_node).unwrap();
        let args = value.split(" ").collect::<Vec<&str>>(); 
        if args.len() > 1 {
            let a = resolve_values(lookup, args[0]);
            let b = resolve_values(lookup, args[2]);
            match value {
                v if v.contains("+") => { a + b },
                v if v.contains("-") => { a - b },
                v if v.contains("*") => { a * b },
                v if v.contains("/") => { a / b },
                _ => {panic!("Unexpected line: {}", value)}
            }
        } else {
            value.parse::<i64>().unwrap()
        }
    }

    fn resolve_inverse(lookup:&HashMap::<&str,&str>, lines: &Vec<String>, current_node:&str) -> i64 {
        // Find the line that contains the node on the right
        let position = lines.iter().position(|l| {
            let right = l.chars().position(|c| c ==':').unwrap();
            l[right..].contains(current_node)}).unwrap();

        let elements = lines[position].split(": ").collect::<Vec<&str>>();
        let args = elements[1].split(" ").collect::<Vec<&str>>(); 

        let node_on_left = current_node == args[0];

        let b = if node_on_left {
            resolve_values(lookup, args[2])
        } else {
            resolve_values(lookup, args[0])
        };

        if elements[0] != "root" {
            let a = resolve_inverse(lookup, lines, elements[0]);

            // length of args should always be > 1
            // If the line we're trying to find is 'root', then just return the 'other' side.

            if node_on_left {
                // If the current node is on the left, of the operation
                match elements[1] {
                    v if v.contains("+") => { a - b },
                    v if v.contains("-") => { a + b },
                    v if v.contains("*") => { a / b },
                    v if v.contains("/") => { a * b },
                    _ => {panic!("")}
                }
            } else {
                // If the current node is on the right, of the operation
                match elements[1] {
                    v if v.contains("+") => { a - b },
                    v if v.contains("-") => { b - a },
                    v if v.contains("*") => { a / b },
                    v if v.contains("/") => { b / a },
                    _ => {panic!("")}
                }
            } 
        } else {
            b
        }
    }

    if !second_part {
        resolve_values(&lookup, "root")
    } else {
        resolve_inverse(&lookup, &lines, "humn")
    }
}

fn day22(lines:Vec<String>, second_part:bool, sample:bool) -> i32  {
    // Split part A and B, as the method used for A was too different for B.
    if !second_part {
        day22_parta(lines, second_part, sample)
    } else {
        day22_partb(lines, second_part, sample)
    }
}
fn day22_parta(lines:Vec<String>, _second_part:bool, sample:bool) -> i32  {

    fn next_step(position:&(i32, i32), heading:&usize, width:i32, sample:bool ) -> ((i32, i32),usize) {
        let offsets_vec = vec![(1,0), ( 0, 1), (-1,0), ( 0,-1)];
        let offset = offsets_vec[heading%offsets_vec.len()];

        if sample {
            ((((width*4) + ((position.0 + offset.0) % (width*4))) % (width*4),  
              ((width*3) + ((position.1 + offset.1) % (width*3))) % (width*3)),*heading)
        } else {
            ((((width*3) + ((position.0 + offset.0) % (width*3))) % (width*3),  
              ((width*4) + ((position.1 + offset.1) % (width*4))) % (width*4)),*heading)
        }
    }

    #[derive(PartialEq, Clone, Copy)]
    enum Map { Void = 0, Empty = 1, Wall = 2}

    let mut rows = Vec::new();
    let mut iter = lines.into_iter();
    let mut points = 0;
    while let Some(line) = iter.next() {
        if line == "" {break;}
        let new_row = line.chars()
                      .into_iter()
                      .map(|c| match c {' ' => {Map::Void}, 
                                        '.' => {points+=1; Map::Empty},
                                        '#' => {points+=1; Map::Wall},
                                        _ => {panic!("Invalid input {}",c)},
                                       })
                      .collect::<Vec<Map>>();
        rows.push(new_row);
    }

    // Calculate the cube width from the input.
    let cube_width = ((points/6) as f32).sqrt() as i32;

    rows.resize(4*cube_width as usize,  vec![Map::Void;cube_width as usize]);
    for row in &mut rows {
        row.resize(4*cube_width as usize,  Map::Void);
    }

    let movements = iter.next().unwrap();

    // Get the initial position.
    let mut position = (rows[0].iter().position(|c| *c == Map::Empty).unwrap() as i32,0);
    let mut heading = 0;

    let mut movements_iter = movements.chars().peekable();
    while !movements_iter.peek().is_none() {
        let mut distance = Vec::<char>::new();
        while let Some(digit) = movements_iter.next_if(|c| c.is_digit(10))
        {
            distance.push(digit);
        }

        for _ in 0..distance.iter().collect::<String>().parse::<i32>().unwrap() {
            let (mut next_position,mut heading) = next_step(&position, &heading, cube_width, sample);
            while rows[next_position.1 as usize][next_position.0 as usize] == Map::Void {
                // Find the first 'non empty' position
                (next_position,heading) = next_step(&next_position, &heading, cube_width, sample);
            }

            match rows[next_position.1 as usize][next_position.0 as usize] {
                Map::Void => {panic!("Void shouldn't be possible here.");},
                Map::Empty => {position = next_position;},
                Map::Wall => {()},
            }
        }

        match movements_iter.next() {
            Some('R') => {heading = (heading + 1) % 4;}, // Get 'next' to turn right.
            Some('L') => {heading = (heading + 3) % 4;}, // turn left (3 rights).
            None => {()} 
            _ => {panic!("Unexpected movement");} 
        }
    }
    
    let result = 1000 * (position.1+1) + 4 * (position.0+1) + heading as i32;

    result
}

fn day22_partb(lines:Vec<String>, _second_part:bool, _sample:bool) -> i32  {

    // Flood fill
    // If leaving a 'panel', update 'dir' and 'normal'
    // Store each pixel in 3d with normal (normal, (x,y,z)) (there will be duplicates on edges).
    
    #[derive(PartialEq, Clone, Copy)]
    enum Map { Void = 0, Empty = 1, Wall = 2}

    let mut rows = Vec::new();
    let mut iter = lines.into_iter();
    let mut points = 0;
    while let Some(line) = iter.next() {
        if line == "" {break;}
        rows.push(line.chars().into_iter().map(|c| { match c {' ' => {Map::Void}, 
                                        '.' => {points +=1; Map::Empty},
                                        '#' => {points +=1; Map::Wall},
                                        _ => {panic!("Invalid input {}",c)},
                                       }}).collect::<Vec<Map>>());
    }

    // Calculate the cube width from the input.
    let cube_width = ((points/6) as f32).sqrt() as i32;

    let panel_pos = (0,0);
    let grid_dir = (1,0);

    // Set the initial position in 3d.
    let mut normal            = (0,0,-1);
    let mut position          = (0,0,0);
    let mut current_direction = (1,0,0);

    // Hash: (normal), (position) = (Map, (x,y))
    let mut real_cube = HashMap::new();
    // Cross product.
    // |  i  j  k |
    // | di dj dk |
    // | ni nj nk |
    fn rotate_anti_clock(dir:(i32,i32,i32), norm:(i32,i32,i32)) -> (i32,i32,i32) {
        ((dir.2 * norm.1) - (dir.1 * norm.2), 
         (dir.0 * norm.2) - (dir.2 * norm.0), 
         (dir.1 * norm.0) - (dir.0 * norm.1))
    }

    // |  i  j  k |
    // | di dj  0 |
    // |  0  0 -1 |
    fn rotate_grid_anti_clock(dir:(i32,i32)) -> (i32,i32) {
        (dir.1, -dir.0)
    }

    fn apply_grid_offset(point:(i32,i32), offset:(i32,i32)) -> (i32,i32) {
        ((point.0 + offset.0), (point.1 + offset.1))
    }

    fn apply_offset(point:(i32,i32,i32), offset:(i32,i32,i32)) -> (i32,i32,i32) {
        ((point.0 + offset.0), (point.1 + offset.1), (point.2 + offset.2))
    }

    fn negate_vector(point:(i32,i32,i32)) -> (i32,i32,i32) {
        (-point.0, -point.1, -point.2)
    }
    
    // Get the initial position in the flat image.
    let grid_position = (rows[0].iter().position(|c| *c == Map::Empty).unwrap() as i32,0);

    // Create the cube in 3d.  Maintain direction for both the current 3d point and the 2d input
    // location.  Insert each input character into a map with the key being 'point + normal', also
    // store the 2d grid (as it's needed at the end to calculate the result values).
    fn build_cube(real_cube: &mut HashMap::<((i32,i32,i32),(i32,i32,i32)),(Map,(i32,i32))>, 
                  grid:&Vec<Vec<Map>>, grid_pos:(i32,i32), panel_pos:(i32,i32), current_grid_dir:(i32,i32), 
                  position:(i32,i32,i32), normal:(i32,i32,i32), current_direction:(i32,i32,i32), width:i32) {
        if None == real_cube.get(&(normal,position)) {
            if panel_pos.0 >= 0 && panel_pos.1 >= 0 && panel_pos.1 < width as i32 && panel_pos.0 < width as i32 
            {
                if grid[grid_pos.1 as usize][grid_pos.0 as usize] == Map::Void {
                    panic!("Grid location invalid.");
                }
                real_cube.insert((normal,position),(grid[grid_pos.1 as usize][grid_pos.0 as usize],(grid_pos.0,grid_pos.1)));
                let mut new_grid_dir = current_grid_dir;
                let mut new_direction = current_direction;
                for _ in 0..4 {
                    // Check each direction.
                    let new_position = apply_offset(position, new_direction);
                    let new_grid_pos = apply_grid_offset(grid_pos, new_grid_dir);
                    let new_panel_pos = apply_grid_offset(panel_pos, new_grid_dir);

                    build_cube(real_cube, grid, new_grid_pos, new_panel_pos, new_grid_dir, 
                               new_position, normal, new_direction, width);
                    new_grid_dir = rotate_grid_anti_clock(new_grid_dir);
                    new_direction = rotate_anti_clock(new_direction, normal);
                }
            } else {
                if grid_pos.0 >= 0 && grid_pos.1 >= 0 && 
                    grid_pos.1 < grid.len() as i32 && grid_pos.0 < grid[grid_pos.1 as usize].len() as i32 &&
                    grid[grid_pos.1 as usize][grid_pos.0 as usize] != Map::Void {
                        // Only continue if the edge of the grid hasn't been reached.
                        let new_position = apply_offset(position, negate_vector(current_direction)); // Need to step back 1 in 3d
                        let new_normal = current_direction;
                        let new_direction = (-normal.0, -normal.1, -normal.2);
                        let new_panel_pos = ((width + grid_pos.0 % width) % width,
                        (width + grid_pos.1 % width) % width);
                        // Switch sides and keep building.
                        build_cube(real_cube, grid, grid_pos, new_panel_pos, current_grid_dir, new_position, new_normal, new_direction, width);
                    }
            }
        }
    }

    build_cube(&mut real_cube, &rows, grid_position, panel_pos, grid_dir, position, normal, current_direction, cube_width);

    // Movement
    let movements = iter.next().unwrap();

    position          = (0,0,0);

    // Answer was too high (36541).
    let mut movements_iter = movements.chars().peekable();
    while !movements_iter.peek().is_none() {
        let mut distance      = Vec::<char>::new();

        while let Some(digit) = movements_iter.next_if(|c| c.is_digit(10))
        {
            distance.push(digit);
        }

        for _ in 0..distance.iter().collect::<String>().parse::<i32>().unwrap() {
            let mut new_normal          = normal;
            let mut new_direction = current_direction;
            let mut new_position = apply_offset(position, new_direction);

            // Check if we need to move to a new surface.
            if new_position.0 < 0 || new_position.1 < 0 || new_position.2 < 0 || 
               new_position.0 >= cube_width || new_position.1 >= cube_width || new_position.2 >= cube_width {
                new_normal = current_direction;
                new_direction = negate_vector(normal);
                new_position = position; // reset the position.
            }

            if let Some((value,_grid_pos)) = real_cube.get(&(new_normal,new_position)) {
                if *value != Map::Wall {
                    current_direction = new_direction;
                    normal = new_normal;
                    position = new_position;
                }
            } else {
                panic!("Can't find location on cube, this shouldn't be possible. {} {} {}", new_position.0, new_position.1, new_position.2);
            }

        }

        match movements_iter.next() {
            Some('R') => {for _ in 0..3{current_direction = rotate_anti_clock(current_direction, normal);}}, // turn right = 3 lefts.
            Some('L') => {current_direction = rotate_anti_clock(current_direction, normal);}, // turn left.
            None => {()} 
            _ => {panic!("Unexpected movement");} 
        }
    }

    // Get the heading
    // Not keeping track of the 2d heading, so move in 3d.  
    // If forward falls of panel, then go in reverse.
    let mut heading_value = 0;
    if let Some((_value,grid_pos)) = real_cube.get(&(normal,position)) {
        let mut grid_direction = (0,0);
        if let Some((_,next_grid_pos)) = real_cube.get(&(normal,apply_offset(position, current_direction))) {
            grid_direction.0 = next_grid_pos.0 - grid_pos.0;
            grid_direction.1 = next_grid_pos.1 - grid_pos.1;
        } else if let Some((_,next_grid_pos)) = real_cube.get(&(normal,apply_offset(position, negate_vector(current_direction)))) {
            grid_direction.0 = grid_pos.0 - next_grid_pos.0;
            grid_direction.1 = grid_pos.1 - next_grid_pos.1;
        }
        // The 'heading' value is right = 0, down = 1, left = 2, up = 3.
        for (i,v) in [(1,0),(0,1),(-1,0),(0,-1)].iter().enumerate() {
            if *v == grid_direction {
                heading_value = i as i32;
            }
        }

    }

    if let Some((_,grid_pos)) = real_cube.get(&(normal,position)) {
        (1000 * (grid_pos.1+1)) + (4 * (grid_pos.0+1)) + heading_value
    } else {
        0
    }
}

fn day23(lines:Vec<String>, second_part:bool) -> i32  {

    let mut elves:Vec<(i32, i32)> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, location) in line.chars().into_iter().enumerate() {
            if location == '#' {
                elves.push((x as i32,y as i32));
            }
        }
    }

    let move_directions = vec![(0,-1), (0,1),(-1,0),(1,0)];
    let mut move_direction_iter = move_directions.iter().cycle();


    let max_rounds = if !second_part {10} else {10000};
    let mut final_round = 0;

    print_points(&elves);

    for round in 0..max_rounds {

        let mut proposed_locations:HashMap::<(i32,i32),u32> = HashMap::new();

        fn apply_offset(point:(i32,i32), offset:(i32,i32)) -> (i32,i32) {
            ((point.0 + offset.0), (point.1 + offset.1))
        }

        let mut proposed_new_elves:Vec<(i32, i32)> = Vec::new();

        // Look for a spot.
        for (pos, elf) in elves.iter().by_ref().enumerate() {
            let mut elf_move_direction = move_direction_iter.clone(); 

            proposed_new_elves.insert(pos, *elf);

            let mut no_other_elves = true;
            for i in -1..=1 {
                for j in -1..=1 {
                    if j != 0 || i != 0 {
                        no_other_elves &= !elves.contains(&apply_offset(*elf,(i, j)));
                    }
                }
            }

            if !no_other_elves {
                for _ in 0..move_directions.len() {
                    let look_dir = elf_move_direction.next().unwrap();
                    let mut other_elves = false;
                    for j in -1..=1 {
                        if look_dir.0 == 0 {
                            other_elves |= elves.contains(&apply_offset(*elf,(j, look_dir.1)));
                        } else {
                            other_elves |= elves.contains(&apply_offset(*elf,(look_dir.0, j)));
                        }
                    }

                    if !other_elves {
                        let new_position = apply_offset(*elf, *look_dir);
                        proposed_new_elves.insert(pos, new_position);
                        if let Some(value) = proposed_locations.get(&new_position)
                        {
                            proposed_locations.insert(new_position, value + 1);
                        } else {
                            proposed_locations.insert(new_position, 1);
                        }
                        break;
                    }
                }
            }
        }


        let mut new_elves:Vec<(i32, i32)> = Vec::new();
        // Move if there was only one elf going there (you).
        for (elf, new_position) in std::iter::zip(&elves, proposed_new_elves) {
            if let Some(value) = proposed_locations.get(&new_position)
            {
                if *value == 1 {
                    new_elves.push(new_position);
                } else {
                    new_elves.push(*elf);
                }
            } else {
                // Elf didn't move.
                new_elves.push(*elf);
            }
        }

        elves = new_elves;

        move_direction_iter.next().unwrap(); 
        if 0 == proposed_locations.len() {
            final_round = round + 1 as i32;
            break;
        }

        print_points(&elves);
    }

    let mut min_x = elves[0].0;
    let mut max_x = elves[0].0;
    let mut min_y = elves[0].1;
    let mut max_y = elves[0].1;
    min_x = elves.iter().fold(min_x, |s, p| std::cmp::min(s, p.0));
    max_x = elves.iter().fold(max_x, |s, p| std::cmp::max(s, p.0));
    max_y = elves.iter().fold(max_y, |s, p| std::cmp::max(s, p.1));
    min_y = elves.iter().fold(min_y, |s, p| std::cmp::min(s, p.1));

    if !second_part {
        ((1+max_x) - min_x) * ((1+max_y) - min_y) - elves.len() as i32
    } else {
        final_round
    }
}

fn day24(lines:Vec<String>, second_part:bool) -> i32  {
    // Horizontal and vertical don't interact.
    // for each step, check the next path.
    // Horizontal repeats after 'x'
    // Vertical repeats after 'y'
    // Empty when both horizontal and vertical are empty

    let mut left:VecDeque<VecDeque<bool>> = VecDeque::new();  // [row][column]
    let mut right:VecDeque<VecDeque<bool>> = VecDeque::new(); // [row][column]
    let mut up:VecDeque<VecDeque<bool>> = VecDeque::new();    // [row][column]
    let mut down:VecDeque<VecDeque<bool>> = VecDeque::new();  // [row][column]

    for (_, line) in lines.iter().enumerate() {
        let mut left_row = VecDeque::new();
        let mut right_row = VecDeque::new();
        let mut up_row = VecDeque::new();
        let mut down_row = VecDeque::new();
        for (_, location) in line.chars().into_iter().enumerate() {
            let (mut is_right, mut is_left, mut is_up, mut is_down) = (false, false, false, false);
            let mut ignore = false;
            match location {
                '>' => { is_right = true;},
                '<' => { is_left = true;},
                '^' => { is_up = true;},
                'v' => { is_down = true;},
                '.' => { },
                 _  => { ignore = true;},
            }
            if !ignore {
                // Push if it's not a '#'.
                left_row.push_back(is_left); 
                right_row.push_back(is_right);
                up_row.push_back(is_up); 
                down_row.push_back(is_down); 
            }
        }
        left.push_back(left_row);
        right.push_back(right_row);
        up.push_back(up_row);
        down.push_back(down_row);
    }
    left.pop_front();
    left.pop_back();
    right.pop_front();
    right.pop_back();
    up.pop_front();
    up.pop_back();
    down.pop_front();
    down.pop_back();

    fn check_is_empty(left:&VecDeque<VecDeque<bool>>, right:&VecDeque<VecDeque<bool>>, up:&VecDeque<VecDeque<bool>>, down:&VecDeque<VecDeque<bool>>, x:i32, y:i32, time:i32) -> bool
    {
        let y_size = right.len() as i32;
        let x_size = right[0].len()as i32;
        if x == x_size - 1 && y == y_size {
            return true;
        }
        if x == 0 && y == -1 {
            return true;
        }
        if x < 0 || y < 0 || x >= x_size || y >= y_size {
            return false;
        }
        !(right[y as usize][((x_size + ((x - time) % x_size)) % x_size) as usize] || 
        left[y as usize][((x_size + ((x + time) % x_size)) % x_size) as usize] || 
        up[((y_size + ((y + time) % y_size)) % y_size) as usize][x as usize] || 
        down[((y_size + ((y - time) % y_size)) % y_size) as usize][x as usize])
    }

    fn apply_offset(point:(i32,i32), offset:(i32,i32)) -> (i32,i32) {
        ((point.0 + offset.0), (point.1 + offset.1))
    }

    let mut quickest_map:HashMap<(i32,i32),i32> = HashMap::new();

    fn search_depth(quickest_map:&mut HashMap<(i32,i32),i32>, left:&VecDeque<VecDeque<bool>>, right:&VecDeque<VecDeque<bool>>, 
                    up:&VecDeque<VecDeque<bool>>, down:&VecDeque<VecDeque<bool>>, 
                    location:(i32, i32), dst:(i32,i32), current_time:i32, time_remaining:i32) -> (bool,i32) {
        let check_directions = vec![(1,0),(0,1),(0,0),(0,-1),(-1,0)]; // x,y
                                                                      
        if let Some(_last_distance) = quickest_map.get(&location)
        {
        } else {
            quickest_map.insert(location, current_time);
        }

        if location == dst {
            return (true, current_time);
        }

        let mut found = false;
        let mut found_time = current_time;
        
        if time_remaining != 0 {
            for check in check_directions.iter() {
                let new_location = apply_offset(location, *check);
                if check_is_empty(&left, &right, &up, &down, new_location.0, new_location.1, current_time+1) {
                    if !found {
                        (found, found_time) = search_depth(quickest_map, left, right, up, down, new_location, dst, current_time+1, time_remaining-1)
                    }
                }
            }
        }
        (found, found_time)
    }

    let max_lookahead = 12; // Fudge factor, not sure what the best look ahead is (appears to give matching results from 7+).
    let mut first_goal_time = 0;
    search_depth(&mut quickest_map, &left, &right, &up, &down, (0, -1), (left[0].len() as i32 -1,left.len() as i32 -1), 0, max_lookahead);
    'outer: for i in 0..10000 {
        for (location,distance) in quickest_map.clone() {
            if i == distance {
                let (found,time) = search_depth(&mut quickest_map, &left, &right, &up, &down, location, (left[0].len() as i32 -1,left.len() as i32 -1), i, max_lookahead);
                if found {
                    println!("{} {}", found, time + 1);
                    first_goal_time = time + 1;
                    break 'outer;
                }
            }
        }
    }

    let mut second_goal_time = 0;
    quickest_map.clear();
    search_depth(&mut quickest_map, &left, &right, &up, &down, (left[0].len() as i32 -1,left.len() as i32), (0,0), first_goal_time, max_lookahead);
    'outer: for i in first_goal_time..10000 {
        for (location,distance) in quickest_map.clone() {
            if i == distance {
                let (found,time) = search_depth(&mut quickest_map, &left, &right, &up, &down, location, (0,0), i, 10);
                if found {
                    println!("{} {}", found, time + 1);
                    second_goal_time = time + 1;
                    break 'outer;
                }
            }
        }
    }

    let mut third_goal_time = 0;
    quickest_map.clear();
    search_depth(&mut quickest_map, &left, &right, &up, &down, (0, -1), (left[0].len() as i32 -1,left.len() as i32 -1), second_goal_time, max_lookahead);
    'outer: for i in second_goal_time..10000 {
        for (location,distance) in quickest_map.clone() {
            if i == distance {
                let (found,time) = search_depth(&mut quickest_map, &left, &right, &up, &down, location, (left[0].len() as i32 -1,left.len() as i32 -1), i, max_lookahead);
                if found {
                    println!("{} {}", found, time + 1);
                    third_goal_time = time + 1;
                    break 'outer;
                }
            }
        }
    }
    println!("1st: {} 2nd: {} 3rd: {}", first_goal_time, second_goal_time, third_goal_time);


    if false { // debug
        for i in 0..16 {
            println!("Minute: {}", i);
            print_blizzard(&left, &right, &up, &down);

            for row_idx in 0..right.len() {
                right[row_idx].rotate_right(1);
                left[row_idx].rotate_left(1);
            }

            up.rotate_left(1);
            down.rotate_right(1);
        }

        print_blizzard(&left, &right, &up, &down);
    }

    fn print_blizzard(left:&VecDeque<VecDeque<bool>>, right:&VecDeque<VecDeque<bool>>, up:&VecDeque<VecDeque<bool>>, down:&VecDeque<VecDeque<bool>>) {
        for y in 0..left.len() {
            for x in 0..left[0].len() {
                let sum = left[y][x] as i32 + right[y][x] as i32 + up[y][x] as i32 + down[y][x] as i32;
                if sum > 1 { print!("{}", sum); } 
                else if left[y][x] { print!("<"); } 
                else if right[y][x] { print!(">"); } 
                else if up[y][x] { print!("^"); } 
                else if down[y][x] { print!("v"); } 
                else { print!("."); } 
            }
            println!("");
        }
        println!("");
    }

    if !second_part {first_goal_time} else {third_goal_time}
}

fn day25(lines:Vec<String>, second_part:bool) -> String  {

    fn snafu_char_to_dec(c:char) -> i64 {
        match c {
            '2' => {2},
            '1' => {1},
            '0' => {0},
            '-' => {-1},
            '=' => {-2},
            _ => {panic!("Unexpected snafu character {}", c);},
        }
    }

    fn snafu_to_dec(line:&String) -> i64 {
        line.chars().fold(0, |sum,c| sum * 5 + snafu_char_to_dec(c))
    }

    fn dec_to_snafu(input: i64) -> String {
        // convert to base 5
        let mut remainder = input;
        let current_mod = 5;
        let mut snafu_digits = Vec::new();
        while remainder != 0 {
            snafu_digits.push(remainder % current_mod);
            remainder = (remainder - (remainder % current_mod))/current_mod;
        }
        let mut fix_snafu = vec![0;snafu_digits.len() + 1];
        for (i, snafu_digit) in snafu_digits.iter().enumerate() {
            // Loop over each digit, if the digit is too high, add to the adjacent digit and
            // subtract in current digit.
            if (fix_snafu[i] + snafu_digit) > 2 {
                fix_snafu[i] =  (fix_snafu[i] + snafu_digit) - 5;
                fix_snafu[i+1] =  1;
            } else {
                fix_snafu[i] = fix_snafu[i] + snafu_digit;
            }
        }
        fix_snafu.reverse();
        let mut snafu_string = String::new();

        for snafu_digit in fix_snafu {
            snafu_string = format!("{}{}", snafu_string, 
                match snafu_digit {
                    -2 => {'='},
                    -1 => {'-'},
                    0 => {'0'},
                    1 => {'1'},
                    2 => {'2'},
                    _ => {panic!("Invalid digit to convert {}", snafu_digit);}});
        }
        snafu_string.trim_start_matches('0').to_string()
    }
    assert_eq!("1=11-2", dec_to_snafu(2022)); 
    assert_eq!("1-0---0", dec_to_snafu(12345)); 
    assert_eq!("1121-1110-1=0", dec_to_snafu(314159265)); 
    let snafu_dec = lines.iter().fold(0, |sum,line| sum + snafu_to_dec(line));

    // There is no second part for this puzzle.
    if !second_part { dec_to_snafu(snafu_dec) } else {"".to_string()}
}

fn print_points(points:&Vec<(i32,i32)>) {
    let mut min_x = points[0].0;
    let mut max_x = points[0].0;
    let mut min_y = points[0].1;
    let mut max_y = points[0].1;
    min_x = points.iter().fold(min_x, |s, p| std::cmp::min(s, p.0));
    max_x = points.iter().fold(max_x, |s, p| std::cmp::max(s, p.0));
    max_y = points.iter().fold(max_y, |s, p| std::cmp::max(s, p.1));
    min_y = points.iter().fold(min_y, |s, p| std::cmp::min(s, p.1));

    let mut output = vec![vec![0;(max_x - min_x + 1) as usize];(max_y - min_y + 1) as usize];
    for point in points {
        output[(point.1 - min_y) as usize][(point.0 - min_x) as usize] = 1;
    }
    println!("Offset, min (x,y) = {} {}", min_x, min_y);
    for row in output {
        for c in row {
            if 0 == c { print!(".");
            } else {
                print!("#")
            }
        }
        println!("")
    }
}
    


#[cfg(test)]
mod tests {
    fn test_helper(day:u8, expect:Vec<&str>) {
        let test_order = [(false, true), (false, false), (true,true), (true, false)];
        for (i, test_mode) in test_order.iter().enumerate() {
            assert_eq!(super::call_day_func(day, test_mode.0, test_mode.1),  expect[i]);
        }
    }

    #[test]
    fn test_day1() { test_helper(1, vec!["24000",          "71934",         "45000",         "211447"]); }
    #[test]
    fn test_day2() { test_helper(2, vec![   "15",          "13268",            "12",          "15508"]); }
    #[test]
    fn test_day3() { test_helper(3, vec![  "157",           "8109",            "70",           "2738"]); }
    #[test]
    fn test_day4() { test_helper(4, vec![    "2",            "507",             "4",            "897"]); }
    #[test]
    fn test_day5() { test_helper(5, vec![  "CMZ",      "TQRFCBSJJ",           "MCD",      "RMHFJNVFP"]); }
    #[test]
    fn test_day6() { test_helper(6, vec![   "11",           "1134",            "26",           "2263"]); }
    #[test]
    fn test_day7() { test_helper(7, vec!["95437",         "919137",      "24933642",        "2877389"]); }
    #[test]
    fn test_day8() { test_helper(8, vec![   "21",           "1798",             "8",         "259308"]); }
    #[test]
    fn test_day9() { test_helper(9, vec![   "13",           "6236",             "1",           "2449"]); }
    #[test]
    fn test_day10() {
        assert_eq!(super::call_day_func(10, false,  true),          "13140");
        assert_eq!(super::call_day_func(10, false, false),          "11960");
        assert_eq!(super::call_day_func(10,  true,  true),  "\n##..##..##..##..##..##..##..##..##..##..\n\
                                                               ###...###...###...###...###...###...###.\n\
                                                               ####....####....####....####....####....\n\
                                                               #####.....#####.....#####.....#####.....\n\
                                                               ######......######......######......####\n\
                                                               #######.......#######.......#######.....");
        assert_eq!(super::call_day_func(10,  true, false),  "\n####...##..##..####.###...##..#....#..#.\n\
                                                               #.......#.#..#.#....#..#.#..#.#....#..#.\n\
                                                               ###.....#.#....###..#..#.#....#....####.\n\
                                                               #.......#.#....#....###..#.##.#....#..#.\n\
                                                               #....#..#.#..#.#....#....#..#.#....#..#.\n\
                                                               ####..##...##..#....#.....###.####.#..#.");
    }

    #[test]
    fn test_day11() { test_helper(11,vec!["10605",          "61503",    "2713310158",    "14081365540"]); }
    #[test]
    fn test_day12() { test_helper(12,vec![   "31",            "440",            "29",            "439"]); }
    #[test]
    fn test_day13() { test_helper(13,vec![   "13",           "6568",           "140",          "19493"]); }
    #[test]
    fn test_day14() { test_helper(14,vec![   "24",            "793",            "93",          "24166"]); }
    #[test]
    fn test_day15() { test_helper(15,vec![   "26",        "5832528",      "56000011", "13360899249595"]); }
    #[test]
    fn test_day16() { test_helper(16,vec![ "1651",           "2359",          "1707",           "2999"]); }
    #[test]
    fn test_day17() { test_helper(17,vec![ "3068",           "3127", "1514285714288",  "1542941176480"]); }
    #[test]
    fn test_day18() { test_helper(18,vec![   "64",           "4242",            "58",           "2428"]); }
    #[test]
    fn test_day19() { test_helper(19,vec![   "33",           "1834",          "3472",           "2240"]); }
    #[test]
    fn test_day20() { test_helper(20,vec![    "3",           "3473",    "1623178306",  "7496649006261"]); }
    #[test]
    fn test_day21() { test_helper(21,vec![  "152", "81075092088442",           "301",  "3349136384441"]); }
    #[test]
    fn test_day22() { test_helper(22,vec![ "6032",          "31568",          "5031",          "36540"]); }
    #[test]
    fn test_day23() { test_helper(23,vec![  "110",           "3862",            "20",            "913"]); }
    #[test]
    fn test_day24() { test_helper(24,vec![   "18",            "297",            "54",            "856"]); }
    #[test]
    fn test_day25() { test_helper(25,vec!["2=-1=0","2=1-=02-21===-21=200",        "",               ""]); }

}
