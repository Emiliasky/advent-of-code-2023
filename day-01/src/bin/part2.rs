use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let hashmap: HashMap<&str, &str> = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e")
    ]);
    let mut modified_input = input.to_string();

    for (key, value) in hashmap {
        modified_input = modified_input.replace(&key, &value);
    }
    let mut result: u32 = 0;
    for line in modified_input.lines() {
        let digits = &line.chars().filter(|c| c.is_digit(10)).collect::<String>();
        result += (digits.chars().nth(0).unwrap().to_string() + &digits.chars().rev().nth(0).unwrap().to_string()).parse::<u32>().unwrap();
    }
    result

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(include_str!("./test2.txt"));
        assert_eq!(result, 281);
    }
}