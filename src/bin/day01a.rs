const INPUT: &str = include_str!("../../inputs/day01a.txt");

fn main() {
    let mut even = Vec::with_capacity(1000);
    let mut odd = Vec::with_capacity(1000);

    INPUT
        .split_whitespace()
        .enumerate()
        .for_each(|(i, x)| match i % 2 == 0 {
            true => even.push(x.parse::<u64>().unwrap()),
            false => odd.push(x.parse::<u64>().unwrap()),
        });

    even.sort_unstable();
    odd.sort_unstable();

    let difference: u64 = odd.iter().zip(even).map(|(x, y)| x.abs_diff(y)).sum();

    println!("The difference between the sorted lists is: {}", difference);
}
