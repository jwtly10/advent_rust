use std::collections::HashMap;

fn main() {
    let input = include_str!("day_01_input.txt");
    process_part_one(input);

    let input2 = include_str!("day_01_input.txt");
    process_part_two(input2);
}

fn process_part_two(input: &str) -> i32 {
    let digit_map: HashMap<&str, &str> = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut result = 0;

    for line in input.lines() {
        println!("Line: {}", line);
        let mut res_vec: Vec<String> = vec![];
        let chars = line.chars();
        let mut running_string = String::new();
        for c in chars {
            running_string.push(c);
            if c.is_numeric() {
                res_vec.push(c.to_string());
            } else {
                for (key, value) in &digit_map {
                    if running_string.ends_with(key) {
                        res_vec.push(value.to_string());
                    }
                }
            }
        }

        let loc_res = res_vec[0].to_string() + &res_vec[res_vec.len() - 1].to_string();
        println!("Loc Res: {}", loc_res);
        result += loc_res.parse::<i32>().unwrap();
    }

    println!("Part 2 Result: {:?}", result);
    return result;
}

fn process_part_one(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let chars = line.chars();
        let mut integers: Vec<i32> = vec![];
        for c in chars {
            if c.is_numeric() {
                integers.push(c.to_digit(10).unwrap() as i32);
            }
        }

        let two_digit_sum = integers[0].to_string() + &integers[integers.len() - 1].to_string();

        // println!("{} Res: {}", line, two_digit_sum);
        result += two_digit_sum.parse::<i32>().unwrap();
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part_two() {
        let input = include_str!("day_01_input_test.txt");
        process_part_two(input);

        assert_eq!(process_part_two(input), 183);
    }
}
