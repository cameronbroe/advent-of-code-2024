use utils::load_input;

const INPUT_FILE_PART_ONE: &str = "inputs/day1.txt";

struct Day1Input {
    left_list: Vec<i32>,
    right_list: Vec<i32>,
}

fn parse_input(input: String) -> Day1Input {
    let mut input_struct: Day1Input = Day1Input {
        left_list: Vec::new(),
        right_list: Vec::new(),
    };

    let input_lines = input.lines();
    for line in input_lines {
        let split: Vec<&str> = line.split("   ").collect();
        let left_number = split[0].parse::<i32>().unwrap();
        let right_number = split[1].parse::<i32>().unwrap();

        input_struct.left_list.push(left_number);
        input_struct.right_list.push(right_number);
    }

    return input_struct;
}

fn diff(x: i32, y: i32) -> i32 {
    return (x - y).abs();
}

fn day1_part1(input: String) -> String {
    let mut parsed_input = parse_input(input);
    parsed_input.left_list.sort();
    parsed_input.right_list.sort();

    let mut diffs: Vec<i32> = Vec::new();
    for ind in 0..parsed_input.left_list.len() {
        let left_val = parsed_input.left_list[ind];
        let right_val = parsed_input.right_list[ind];

        let diff = diff(left_val, right_val);
        diffs.push(diff);
    }

    let sum = diffs.into_iter().reduce(|x, y| x + y).unwrap();
    return sum.to_string();
}

fn main() {
    let input = load_input(INPUT_FILE_PART_ONE.to_string());
    let part1_output = day1_part1(input);
    println!("part 1: {}", part1_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_FILE_PART_ONE: &str = "sample_input.txt";

    #[test]
    fn day1_sample_passes() {
        let input = load_input(SAMPLE_INPUT_FILE_PART_ONE.to_string());
        let output = day1_part1(input);
        assert_eq!(output.to_owned(), "11");
    }
}
