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
            9 => {format!("{}", day9(lines, second_part))},
           10 => {format!("{}", day10(lines, second_part))},
           11 => {format!("{}", day11(lines, second_part))},
           12 => {format!("{}", day12(lines, second_part))},
           13 => {format!("{}", day13(lines, second_part))},
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
        assert_eq!(super::call_day_func(9, false),      "6236");
        assert_eq!(super::call_day_func(9, true),      "2449");
        assert_eq!(super::call_day_func(10, false),    "11960");
        assert_eq!(super::call_day_func(10, true),     "\n####...##..##..####.###...##..#....#..#.\n\
                                                          #.......#.#..#.#....#..#.#..#.#....#..#.\n\
                                                          ###.....#.#....###..#..#.#....#....####.\n\
                                                          #.......#.#....#....###..#.##.#....#..#.\n\
                                                          #....#..#.#..#.#....#....#..#.#....#..#.\n\
                                                          ####..##...##..#....#.....###.####.#..#.");
        assert_eq!(super::call_day_func(11, false),    "61503");
        assert_eq!(super::call_day_func(11, true), "14081365540");
        assert_eq!(super::call_day_func(12, false),      "440");
        assert_eq!(super::call_day_func(12, true),       "439");
        assert_eq!(super::call_day_func(13, false),      "6568");
        assert_eq!(super::call_day_func(13, true),       "19493");
    }
}
