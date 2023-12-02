
pub fn run(contents: String) {
    let lines = contents.split("\n");
    let mut result = 0;
    for line in lines.enumerate() {
        let temp_result = find_result(line.1.to_string());
        println!("Input: {} Result: {}", line.1, temp_result);
        result += temp_result
    }
    print!("{}", result)
}

fn find_result(input: String) -> i32 {
    let digits = find_digits(input);
    let first = digits.first().unwrap().0;
    let last = digits.last().unwrap().0;

    first * 10 + last
}

const DIGIT_PATTERNS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const DIGIT_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn find_digits(input: String) -> Vec<(i32, usize)> {
    let mut found_digits: Vec<(i32, usize)> = vec![];
    for digit in DIGIT_PATTERNS {
        let occurences: Vec<_> = input.match_indices(digit).collect();

        occurences.iter().for_each(|o| {
            found_digits.push((string_to_i32(o.1), o.0));
        });
    }

    for digit in DIGIT_CHARS {
        let occurences: Vec<_> = input.match_indices(digit).collect();

        occurences.iter().for_each(|o| {
            found_digits.push((char_to_i32(digit), o.0));
        });
    }

    found_digits.sort_by(|lhs, rhs| lhs.1.cmp(&rhs.1));
    found_digits
}

fn string_to_i32(input: &str) -> i32 {
    return match DIGIT_PATTERNS.iter().position(|&r| r == input) {
        Some(i) => (i as i32) + 1,
        None => panic!(),
    };
}

fn char_to_i32(input: char) -> i32 {
    input as i32 - 0x30
}

#[cfg(test)]
mod tests {
    use crate::day1::find_result;

    use super::find_digits;

    #[test]
    fn test_from_aoc() {
        assert_eq!(find_result("two1nine".to_string()), 29);
        assert_eq!(find_result("eightwothree".to_string()), 83);
        assert_eq!(find_result("abcone2threexyz".to_string()), 13);
        assert_eq!(find_result("xtwone3four".to_string()), 24);
        assert_eq!(find_result("4nineeightseven2".to_string()), 42);
        assert_eq!(find_result("zoneight234".to_string()), 14);
        assert_eq!(find_result("7pqrstsixteen".to_string()), 76);
        assert_eq!(find_result("8twosvdmcntf1hfive393".to_string()), 83);
    }

    #[test]
    fn test_find_digits() {
        let input_1 = "one2three45";
        let result_1 = find_digits(input_1.to_string());
        assert_eq!(result_1.len(), 5);
        assert_eq!(result_1, [(1, 0), (2, 3), (3, 4), (4, 9), (5, 10)]);
    }

    #[test]
    fn test_char_to_i32() {
        use crate::day1::char_to_i32;

        assert_eq!(char_to_i32('0'), 0);
        assert_eq!(char_to_i32('1'), 1);
        assert_eq!(char_to_i32('2'), 2);
        assert_eq!(char_to_i32('3'), 3);
        assert_eq!(char_to_i32('4'), 4);
        assert_eq!(char_to_i32('5'), 5);
        assert_eq!(char_to_i32('6'), 6);
        assert_eq!(char_to_i32('7'), 7);
        assert_eq!(char_to_i32('8'), 8);
        assert_eq!(char_to_i32('9'), 9);
    }

    #[test]
    fn test_string_to_i32() {
        use crate::day1::string_to_i32;

        assert_eq!(string_to_i32("one"), 1);
        assert_eq!(string_to_i32("two"), 2);
        assert_eq!(string_to_i32("three"), 3);
        assert_eq!(string_to_i32("four"), 4);
        assert_eq!(string_to_i32("five"), 5);
        assert_eq!(string_to_i32("six"), 6);
        assert_eq!(string_to_i32("seven"), 7);
        assert_eq!(string_to_i32("eight"), 8);
        assert_eq!(string_to_i32("nine"), 9);
    }
}
