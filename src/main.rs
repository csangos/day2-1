use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn load_from_file_commands(file_path: &str) -> Vec<(String, i32)> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let commands: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();

    // println!("{:?}", commands[0]);

    let moves: Vec<(String, i32)> = commands
        .iter()
        .map(|command| {
            let mut parts = command.split_whitespace();
            let command = parts.next().unwrap();
            let value = parts.next().unwrap().parse::<i32>().unwrap();
            (command.to_string(), value)
        })
        .collect();

    moves
}

fn dive(commands: Vec<(String, i32)>) {
    let mut depth = 0;
    let mut horz = 0;
    for command in &commands {
        if command.0 == "forward" {
            horz += command.1;
        } else if command.0 == "down" {
            depth += command.1;
        } else if command.0 == "up" {
            depth -= command.1;
        }
    }
    println!("Horz: {} Depth {} // Answer: {}", horz, depth, horz * depth);
}

fn main() {
    let commands = load_from_file_commands("src/course.txt");
    dive(commands); // 2-1: calcualte horizontal position and depth given the inputs in
}
