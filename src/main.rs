use std::env;
use std::fs;
use std::time::Instant;

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

fn help() {
    println!("usage:
rustaoc <puzzle_number> <input>
    runs the puzzle for the given input.

");
}

fn main() {
    println!("Welcome to AoC solutions with RUST! (cc 2021)");

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
