use std::fs;
use std::time::Instant;

use aoc2016::utils::bespoke::AssembunnyInterpreter;

const PROBLEM_NAME: &str = "Leonardo's Monorail";
const PROBLEM_INPUT_FILE: &str = "./input/day12.txt";
const PROBLEM_DAY: u64 = 12;

/// Processes the AOC 2016 Day 12 input file and solves both parts of the problem. Solutions are
/// printed to stdout.
pub fn main() {
    let start = Instant::now();
    // Input processing
    let input = process_input_file(PROBLEM_INPUT_FILE);
    let input_parser_timestamp = Instant::now();
    let input_parser_duration = input_parser_timestamp.duration_since(start);
    // Solve part 1
    let p1_solution = solve_part1(&input);
    let p1_timestamp = Instant::now();
    let p1_duration = p1_timestamp.duration_since(input_parser_timestamp);
    // Solve part 2
    let p2_solution = solve_part2(&input);
    let p2_timestamp = Instant::now();
    let p2_duration = p2_timestamp.duration_since(p1_timestamp);
    // Print results
    println!("==================================================");
    println!("AOC 2016 Day {PROBLEM_DAY} - \"{PROBLEM_NAME}\"");
    println!("[+] Part 1: {p1_solution}");
    println!("[+] Part 2: {p2_solution}");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Execution times:");
    println!("[+] Input:  {input_parser_duration:.2?}");
    println!("[+] Part 1: {p1_duration:.2?}");
    println!("[+] Part 2: {p2_duration:.2?}");
    println!(
        "[*] TOTAL:  {:.2?}",
        input_parser_duration + p1_duration + p2_duration
    );
    println!("==================================================");
}

/// Processes the AOC 2016 Day 12 input file in the format required by the solver functions.
/// Returned value is Assembunny interpreter created from the instructions listed in the iput file.
fn process_input_file(filename: &str) -> AssembunnyInterpreter {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    AssembunnyInterpreter::new(&raw_input).unwrap()
}

/// Solves AOC 2016 Day 12 Part 1 // Returns the value held in register 'a' of the Assembunny
/// interpreter after executing the program.
fn solve_part1(interpreter: &AssembunnyInterpreter) -> isize {
    let mut interpreter = interpreter.clone();
    interpreter.execute().unwrap();
    interpreter.get_register('a').unwrap()
}

/// Solves AOC 2016 Day 12 Part 2 // Returns the value held in register 'a' of the Assembunny
/// interpreter after executing the program, with register 'c' initialised to 1.
fn solve_part2(interpreter: &AssembunnyInterpreter) -> isize {
    let mut interpreter = interpreter.clone();
    interpreter.set_register('c', 1).unwrap();
    interpreter.execute().unwrap();
    interpreter.get_register('a').unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 12 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day12_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(318003, solution);
    }

    /// Tests the Day 12 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day12_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(9227657, solution);
    }
}
