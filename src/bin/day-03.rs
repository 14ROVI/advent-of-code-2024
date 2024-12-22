use regex::Regex;
use std::fs;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let content = fs::read_to_string("./inputs/day-03.txt").unwrap();
    let mut total = 0;
    let mut enabled = true;

    for capture in re.captures_iter(&content) {
        if capture.get(0).unwrap().as_str() == "do()" {
            enabled = true;
        } else if capture.get(0).unwrap().as_str() == "don't()" {
            enabled = false;
        } else if enabled {
            let num1 = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let num2 = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
            total += num1 * num2;
        }
    }

    println!("The sum of the multiplications is: {}", total);
}
