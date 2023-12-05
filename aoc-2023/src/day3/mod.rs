
struct ProcessInput {
    current_line: String,
    previous_line: Option<String>,
    next_line: Option<String>,
}
fn find_valid_numbers(input: ProcessInput) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    let char_iterator = input.current_line.chars();

    let mut start_index = 0;
    let mut number_string = String::new();
    let mut adjacent_chars: Vec<char> = vec![];
    let mut last_char: char = '.';

    for (index, character) in char_iterator.enumerate() {
        // add numeric chars to string
        if character.is_numeric() {
            if number_string.len() == 0 {
                adjacent_chars.push(last_char);
                start_index = if index > 0 { index - 1 } else { 0 };
            }
            number_string.push(character);

        } else if number_string.len() > 0 {
            // process the number by verifying its adjacent chars
            adjacent_chars.push(character);
        }

        // if a number was found and the current char is not numeric, process the number
        if number_string.len() > 0 && (!character.is_numeric() || index == input.current_line.len() - 1) {
            let mut end_index = start_index + number_string.len() + 2;
            end_index = if end_index > input.current_line.len()  {
                input.current_line.len() 
            } else {
                end_index 
            };

            if let Some(previous_line) = &input.previous_line {
                let previous_line_slice = &previous_line[start_index..end_index];
                previous_line_slice
                    .chars()
                    .for_each(|c| adjacent_chars.push(c));
            }

            if let Some(next_line) = &input.next_line {
                let next_line_slice = &next_line[start_index..end_index];
                next_line_slice.chars().for_each(|c| adjacent_chars.push(c));
            }


            let mut is_part_number = false; 
            for(_, c) in adjacent_chars.iter().enumerate() {
                if *c != '.' {
                    is_part_number = true;
                    break;
                }
            }

            if is_part_number {
                let number = number_string.parse::<i32>().unwrap();
                result.push(number.try_into().unwrap());
            }

            adjacent_chars.clear();
            number_string.clear();
            start_index = 0;
        }

        last_char = character;
    }
    result
}

pub fn run(contents: String) {
    let lines = contents.split("\n").enumerate();
    let lines_array: Vec<_> = contents.split("\n").enumerate().collect();
    let mut result = 0;
    for (index, line) in lines {
        let mut next_line = None;
        let mut previous_line = None;
        if index != 0 {
            previous_line = Some(lines_array[index - 1].1.to_string());
        }
        if index != lines_array.len() - 1 {
            next_line = Some(lines_array[index + 1].1.to_string());
        }
        let input = ProcessInput {
            current_line: line.to_string(),
            previous_line,
            next_line,
        };
        let valid_numbers = find_valid_numbers(input);
        println!("Valid numbers {}: {:?}", index, valid_numbers);
        valid_numbers.iter().for_each(|n| result += *n);
    }
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::day3::{find_valid_numbers, ProcessInput};

    #[test]
    fn test_find_valid_numbers() {
        let input = ProcessInput {
            current_line: String::from("4+../353........................%55*914........105.................829...........112...............75.....................................55"),
            next_line: None,
            previous_line: /* . */Some("....#......(425.............-923.84*......947......999*..............*....280*...........732...&.....*.............14.............683.....^^".to_string()),
        };

        let numbers = find_valid_numbers(input);
        let expected = vec![4, 353, 55, 914, 829, 75, 55];

        assert_eq!(numbers, expected);
    }
}
