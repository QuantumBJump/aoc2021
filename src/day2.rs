use super::helpers::*;
#[allow(dead_code)]

pub fn run() {
    println!("Day 2:");
    let contents = input("./src/inputs/day2.txt");
    let contents_vec = input_vec(&contents);
    println!("  Part 1:");
    part_1(&contents_vec);

    println!("  Part 2:");
    part_2(&contents_vec);

    println!("");
}

fn part_1(contents: &Vec<&str>) {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for line in contents {
        let command: Vec<_> = line.split(" ").collect();
        let amount = command[1].parse::<i32>().unwrap();
        match command[0] {
            "forward" => {
                horizontal_pos += amount;
            },
            "down" => {
                depth += amount;
            },
            "up" => {
                depth -= amount;
            },
            _ => println!("Error"),
        }
    }

    println!("    Final depth: {}\n    Final distance: {}\n    Multiplied: {}", depth, horizontal_pos, depth*horizontal_pos);
}

fn part_2(contents: &Vec<&str>) {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in contents {
        let command: Vec<_> = line.split(" ").collect();
        let amount = command[1].parse::<i32>().unwrap();
        match command[0] {
            "down" => {
                aim += amount;
            },
            "up" => {
                aim -= amount;
            },
            "forward" => {
                position += amount;
                depth += amount * aim;
            },
            _ => println!("Error"),
        }
    }

    println!("    Final depth: {}", depth);
    println!("    Final position: {}", position);
    println!("    Multiplied: {}", depth * position);
}