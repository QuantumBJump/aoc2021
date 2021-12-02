use super::helpers::*;
#[allow(dead_code)]


pub fn run() {
    println!("Day 1:");
    let contents = input("./src/inputs/day1.txt");
    let contents_vec = input_vec(&contents);
    println!("  Part 1:");
    part_1(&contents_vec);

    println!("  Part 2:");
    part_2(&contents_vec);
    println!("");

}

pub fn part_1(contents: &Vec<&str>) {
    let mut depth_increases = 0;
    let mut first = true;
    let mut prev_depth = 0;

    for line in contents {
        if first == true {
            let depth_int = line.parse::<i32>().unwrap();
            first = false;
            prev_depth = depth_int;
        } else {
            let depth_int = line.parse::<i32>().unwrap();
            if depth_int > prev_depth {
                depth_increases += 1;
            }
            prev_depth = depth_int;
        }
    }
    println!("    Total increases: {}", depth_increases);
}

fn sum(a: &[&str]) -> i32 {
    let mut total = 0;
    for item in a {
        let item_int = item.parse::<i32>().unwrap();
        total += item_int;
    }
    total
}

pub fn part_2(contents: &Vec<&str>) {
    let mut next_windex = 1;
    let mut number_increases = 0;

    let current_window = &contents[0..3];
    let mut current_sum = sum(current_window);
    // println!("\t{} (N/A - no previous sum)", current_sum);

    loop {
        if next_windex+2 >= contents.len() {
            break;
        }
        // let mut change = "no change";
        let next_window = &contents[next_windex..next_windex+3];
        let next_sum = sum(next_window);
        if next_sum > current_sum {
            number_increases += 1;
            // change = "increased";
        } else if next_sum < current_sum {
            // change = "decreased";
        }
        // println!("\t{} ({})", next_sum, change);
        // Move forwards
        current_sum = next_sum;
        next_windex += 1;
    }

    println!("    Total increases: {}", number_increases)
}