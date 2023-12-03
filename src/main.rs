use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_increases_given() {
        let input: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(number_of_increase(&input), 7);
    }

    #[test]
    fn sliding_window_given() {
        let input: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(sliding_window(&input), 5);
    }

    #[test]
    fn calculate_displacement_given() {
        let input: Vec<String> = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"].iter().map(|&s| s.to_string()).collect();
        assert_eq!(calculate_displacement(&input), 150);
    }

    #[test]
    fn reverse_string() {
        let input: &str = "This";
        assert_eq!(input.chars().rev().collect::<String>(), "sihT");
    }

    #[test]
    fn calculate_calibration_given() {
        let input: Vec<String> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"].iter().map(|&s| s.to_string()).collect();
        assert_eq!(calculate_calibration(&input), 142);
    }

    #[test]
    fn min_max_dict() {
        let numbers = HashMap::from([
            (0, "zero"),
            (1, "one"),
            (2, "two"),
            (3, "three")
        ]);
        assert_eq!(numbers.keys().max().unwrap_or(&0), &3);
        assert_eq!(numbers.keys().min().unwrap_or(&0), &0);
    }
}

fn number_of_increase(numbers: &Vec<i32>) -> i32 {

    let values: i32 = numbers.windows(2)
        .map(|s| (s[0] < s[1]) as i32)
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    return values;
}

fn number_of_increase_filt(numbers: &Vec<i32>) -> usize {

    let values: usize = numbers.windows(2)
        .filter(|s| (s[0] < s[1]))
        .collect::<Vec<_>>()
        .len();

    return values;
}

fn sliding_window(numbers: &Vec<i32>) -> usize {

    let values: Vec<i32> = numbers.windows(3)
        .map(|s| (s[0] + s[1] + s[2]))
        .collect::<Vec<_>>();
    let result: usize = number_of_increase_filt(&values);

    return result;
}

fn sonar_sweep(input: &String){
    println!("Running a sonar sweep through {}", input);
    println!(".... beeep .... beeep .... beeep ...");
    let content = fs::read_to_string(input)
        .expect("Something went wrong reading the file");

    let lines: Vec<i32> = content.lines()
        .map(|s| s.parse::<i32>().expect("this doesn't look like a number"))
        .collect();

    println!("{} data points", lines.len());
    println!("First data point: {}", lines[0]);
    println!("Last data point: {}", lines[lines.len() - 1]);

    let mut before = Instant::now();
    let values_filt: usize = number_of_increase_filt(&lines);
    println!("Filter (count bools) time: {:.2?}", before.elapsed());

    before = Instant::now();
    let values: i32 = number_of_increase(&lines);
    println!("Map (sum bools) time: {:.2?}", before.elapsed());

    assert_eq!(values_filt, values as usize);

    println!("Number of increases for part 1: {}", values);

    let values_2: usize = sliding_window(&lines);
    println!("Number of increases for part 2: {}", values_2);

}

fn calculate_displacement(lines: &Vec<String>) -> i32 {

    let mut depth = 0;
    let mut position = 0;

    for line in lines {
        let action_dx: Vec<&str> = line.split_whitespace().collect();
        let action: &str = action_dx[0];
        let dx: i32 = action_dx[1].parse::<i32>().expect("This doesn't look like a number");

        match action {
            "up" => {
                depth += -1 * dx;
            },
            "down" => {
                depth += dx;
            },
            "forward" => {
                position += dx;
            },
            _ => {
                println!("Moving {}", action);
            }
        }

    }

    return depth * position;
}

fn dive(input: &String){
    println!("Moving following the instructions you provide through {}", input);
    println!(".... weee  .... dooown .... up ...");
    let content = fs::read_to_string(input)
        .expect("Something went wrong reading the file");

    let lines: Vec<String> = content.lines()
        .map(|s| s.parse::<String>().expect("Is there a problem?"))
        .collect::<Vec<_>>();


    let displacement = calculate_displacement(&lines);

    println!("Final displacement {}", displacement);

}

fn calculate_calibration(lines: &Vec<String>) -> u32 {
    let mut values: Vec<u32> = Vec::new();
    let mut val1: u32 = 0;
    let mut val2: u32 = 0;
    let numbers = HashMap::from([
        ("zero", 0 as u32),
        ("one", 1 as u32),
        ("two", 2 as u32),
        ("three", 3 as u32),
        ("four", 4 as u32),
        ("five", 5 as u32),
        ("six", 6 as u32),
        ("seven", 7 as u32),
        ("eight", 8 as u32),
        ("nine", 9 as u32),
    ]);
    let mut digit: u32 = 0;

    for line in lines {
        let mut positions = HashMap::new();
        for (number_text, number) in &numbers{
            if line.find(number_text).is_some() {
                positions.insert(line.find(number_text), number.clone());
                positions.insert(line.rfind(number_text), number.clone());
            }
        }
        for (index, single_char) in line.chars().enumerate() {
        //for (index, single_char) in(0..).zip(line.chars()) {
            if single_char.is_digit(10) {
                digit = single_char.to_digit(10).unwrap();
                positions.insert(Some(index), digit);
            }
        }

        // printing a hash
        // for (key, value) in &positions {
        //     println!("{:?}: {:?}", key, value);
        // }
        // find min and max of the keys and use them.
        val1 = 10 * positions.get(positions.keys().min().unwrap()).unwrap();
        val2 = positions.get(positions.keys().max().unwrap()).unwrap() * 1;
        values.push(val1 + val2);

    }
    return values
        .iter()
        .sum();
}

fn trebuchet(input: &String){
    println!("Extracting calibration values from {}", input);
    let content = fs::read_to_string(input)
        .expect("Something went wrong reading the file");

    let lines: Vec<String> = content.lines()
        .map(|s| s.parse::<String>().expect("Is there a problem?"))
        .collect::<Vec<_>>();

    let calibration = calculate_calibration(&lines);

    println!("Calibration value {}", calibration);

}

fn help() {
    println!("usage:
rustaoc <puzzle_number> <input>
    runs the puzzle for the given input.

");
}

fn main() {
    println!("Welcome to AoC solutions with RUST! (cc 2021, 2023)");

    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments - only the name of the program
        2 => {
            println!("Probably you should give us an extra file!");
        },
        3 => {
            let puzzle = &args[1];
            let input = &args[2];
            println!("I'm going to run {} with {} as input", puzzle, input);
            let puzzle_number: i32 = match puzzle.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: The puzzle number to execute should be an integer number");
                    return;
                }
            };
            match puzzle_number {
                202101 => {
                    println!("Running puzzle {} with {} as input", puzzle_number, input);
                    sonar_sweep(input);
                },
                202102 => {
                    println!("Running puzzle {} with {} as input", puzzle_number, input);
                    dive(input);
                },
                202301 => {
                    println!("Running puzzle {} with {} as input", puzzle_number, input);
                    trebuchet(input);
                },
                _ => {
                    println!("Puzzle {} hasn't been implemented yet", puzzle_number);
                }
            }
        },
        1 |  _ => {
            help();
        },

    }
}
