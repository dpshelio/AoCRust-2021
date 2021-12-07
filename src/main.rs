use std::env;


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
            }
        },
        1 |  _ => {
            help();
        },

    }
}
