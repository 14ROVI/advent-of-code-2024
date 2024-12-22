use std::fs;

fn is_report_valid(report: Vec<i64>, tolerance: u64) -> bool {
    if report.len() == 1 {
        return false;
    }

    let last_change = report[1] - report[0];

    for i in 0..(report.len() - 1) {
        let change = report[i + 1] - report[i];
        let changed_dir = change.is_positive() != last_change.is_positive();
        let out_of_bound = change.abs() < 1 || 3 < change.abs();

        if changed_dir || out_of_bound {
            if tolerance == 0 {
                return false;
            }

            if i > 0 {
                let mut without_before = report.clone();
                without_before.remove(i - 1);
                if is_report_valid(without_before, tolerance - 1) {
                    return true;
                };
            }

            let mut without_left = report.clone();
            without_left.remove(i);
            if is_report_valid(without_left, tolerance - 1) {
                return true;
            };

            let mut without_right = report.clone();
            without_right.remove(i + 1);
            if is_report_valid(without_right, tolerance - 1) {
                return true;
            };

            return false;
        }
    }

    return true;
}

fn main() {
    let reports: Vec<Vec<i64>> = fs::read_to_string("./inputs/day-02.txt")
        .unwrap()
        .split('\n')
        .map(|s| {
            s.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let part1 = reports
        .clone()
        .into_iter()
        .filter(|report| is_report_valid(report.to_vec(), 0))
        .count();

    let part2 = reports
        .clone()
        .into_iter()
        .filter(|report| is_report_valid(report.to_vec(), 1))
        .count();

    println!("For part 1 there are {} valid reports!", part1);
    println!("For part 2 there are {} valid reports!", part2);
}
