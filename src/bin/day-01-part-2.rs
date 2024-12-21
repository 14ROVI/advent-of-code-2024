const INPUT: &str = include_str!("../../inputs/day-01.txt");

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

    let similarity = even.iter().fold(0, |acc, x| {
        acc + x * (odd.iter().filter(|y| *y == x)).count() as u64
    });

    println!("The similarity between the lists is: {}", similarity);
}
