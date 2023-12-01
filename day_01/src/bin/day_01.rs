fn main() {
    let input = include_str!("day_01_input.txt");
    process_part_one(input);
}

fn process_part_one(input: &str){
    let mut result = 0;
    for line in input.lines() {
        let chars = line.chars();
        let mut integers: Vec<i32> = vec![];
        for c in chars {
            if c.is_numeric(){
                integers.push(c.to_digit(10).unwrap() as i32);
            }
        }

        let two_digit_sum = integers[0].to_string() + &integers[integers.len() - 1].to_string();

        println!("{} Res: {}", line, two_digit_sum);
        result += two_digit_sum.parse::<i32>().unwrap();
    }

    println!("Result: {}", result);
}
