use regex::Regex;
use utils::*;

const INPUT_FILE: &str = "inputs/day3.txt";

fn parse_out_multiplier_calls(input: String) -> Vec<(u32, u32)> {
    let mut multipler_calls = Vec::new();
    let mult_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches_iter = mult_regex.captures_iter(&input).map(|m| m.extract());
    for (_, [x, y]) in matches_iter {
        multipler_calls.push((x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
    }
    return multipler_calls;
}

fn day3_part1(input: String) -> String {
    let multiplier_calls = parse_out_multiplier_calls(input);
    let mut total = 0;
    for call in multiplier_calls {
        total += call.0 * call.1;
    }
    return total.to_string();
}

fn main() {
    let input = load_input(INPUT_FILE.to_string());
    let part1_output = day3_part1(input.clone());
    println!("part 1: {}", part1_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_FILE: &str = "./sample_input.txt";

    #[test]
    fn day3_part1_sample_passes() {
        let input = load_input(SAMPLE_INPUT_FILE.to_string());
        let output = day3_part1(input);
        assert_eq!(output, "161");
    }
}
