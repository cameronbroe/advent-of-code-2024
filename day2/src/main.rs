use utils::*;

const INPUT_FILE: &str = "inputs/day2.txt";

fn is_report_safe(report: String) -> bool {
    let levels: Vec<&str> = report.split(" ").collect();
    let parsed_levels: Vec<u32> = levels
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut is_safe = false;
    let is_sorted = parsed_levels.clone().iter().is_sorted_by(|x, y| x <= y)
        || parsed_levels.clone().iter().is_sorted_by(|x, y| x >= y);
    if is_sorted {
        is_safe = true;
        for i in 0..(parsed_levels.len() - 1) {
            let curr = parsed_levels[i];
            let next = parsed_levels[i + 1];
            let diff = next.abs_diff(curr);
            if diff < 1 || diff > 3 {
                is_safe = false;
                break;
            }
        }
    }
    return is_safe;
}

fn day2_part1(input: String) -> String {
    let mut num_safe: u32 = 0;
    for report in input.lines() {
        if is_report_safe(report.to_string()) {
            num_safe += 1;
        }
    }

    return num_safe.to_string();
}

fn main() {
    let input = load_input(INPUT_FILE.to_string());
    let part1_output = day2_part1(input.clone());
    println!("part 1: {}", part1_output);
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
}
