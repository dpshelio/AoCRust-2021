use std::env;

fn sonar_sweep(input: &String){
    println!("Running a sonar sweep through {}", input);
    println!(".... beeep .... beeep .... beeep ...");
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
                1 => {
                    println!("Running puzzle {} with {} as input", puzzle_number, input);
                    sonar_sweep(input);
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
