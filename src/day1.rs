use std::fs;

fn input() -> String {
    let contents = fs::read_to_string("./src/inputs/day1.txt").unwrap();
    return contents;
}

pub fn run() {
    let contents = input();
    println!("Part 1:");
    part_1(&contents);

    println!("\nPart 2:");
    part_2(&contents);

}

pub fn part_1(contents: &String) {
    let result: Vec<_> = contents.lines().collect();
    let mut depth_increases = 0;
    let mut first = true;
    let mut prev_depth = 0;

    for line in result {
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
    println!("\tTotal increases: {}", depth_increases);
}

fn sum(a: &[&str]) -> i32 {
    let mut total = 0;
    for item in a {
        let item_int = item.parse::<i32>().unwrap();
        total += item_int;
    }
    total
}

pub fn part_2(contents: &String) {
    let list: Vec<_> = contents.lines().collect();
    let mut next_windex = 1;
    let mut number_increases = 0;

    let current_window = &list[0..3];
    let mut current_sum = sum(current_window);
    println!("\t{} (N/A - no previous sum)", current_sum);

    loop {
        if next_windex+2 >= list.len() {
            break;
        }
        let mut change = "no change";
        let next_window = &list[next_windex..next_windex+3];
        let next_sum = sum(next_window);
        if next_sum > current_sum {
            number_increases += 1;
            change = "increased";
        } else if next_sum < current_sum {
            change = "decreased";
        }
        println!("\t{} ({})", next_sum, change);
        // Move forwards
        current_sum = next_sum;
        next_windex += 1;
    }

    println!("\tTotal increases: {}", number_increases)
}