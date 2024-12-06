use utils::*;

const INPUT_FILE: &str = "inputs/day2.txt";

fn is_report_safe(report: &Vec<u32>) -> bool {
    let mut is_safe = false;
    let is_sorted = report.clone().iter().is_sorted_by(|x, y| x <= y)
        || report.clone().iter().is_sorted_by(|x, y| x >= y);
    if is_sorted {
        is_safe = true;
        for i in 0..(report.len() - 1) {
            let curr = report[i];
            let next = report[i + 1];
            let diff = next.abs_diff(curr);
            if diff < 1 || diff > 3 {
                is_safe = false;
                break;
            }
        }
    }
    return is_safe;
}

fn is_safe_with_dampener(report: &Vec<u32>) -> bool {
    let mut is_safe = false;
    for i in 0..report.len() {
        let mut dampened_report = report.clone();
        dampened_report.remove(i);
        if is_report_safe(&dampened_report) {
            is_safe = true;
            break;
        }
    }

    return is_safe;
}

fn day2_part1(input: String) -> String {
    let mut num_safe: u32 = 0;
    for report in input.lines() {
        let levels: Vec<&str> = report.split(" ").collect();
        let parsed_levels: Vec<u32> = levels
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        if is_report_safe(&parsed_levels) {
            num_safe += 1;
        }
    }

    return num_safe.to_string();
}

fn day2_part2(input: String) -> String {
    let mut num_safe: u32 = 0;
    for report in input.lines() {
        let levels: Vec<&str> = report.split(" ").collect();
        let parsed_levels: Vec<u32> = levels
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        if is_report_safe(&parsed_levels) {
            num_safe += 1;
        } else {
            if is_safe_with_dampener(&parsed_levels) {
                num_safe += 1;
            }
        }
    }

    return num_safe.to_string();
}

fn main() {
    let input = load_input(INPUT_FILE.to_string());
    let part1_output = day2_part1(input.clone());
    println!("part 1: {}", part1_output);
    let part2_output = day2_part2(input.clone());
    println!("part 2: {}", part2_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_FILE: &str = "sample_input.txt";

    #[test]
    fn day2_part1_sample_passes() {
        let input = load_input(SAMPLE_INPUT_FILE.to_string());
        let output = day2_part1(input);
        assert_eq!(output.to_owned(), "2");
    }

    #[test]
    fn day2_part2_sample_passes() {
        let input = load_input(SAMPLE_INPUT_FILE.to_string());
        let output = day2_part2(input);
        assert_eq!(output.to_owned(), "4");
    }
}
