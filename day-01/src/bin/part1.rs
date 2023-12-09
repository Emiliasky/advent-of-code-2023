fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut result: u32 = 0;
    for line in input.lines() {
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
        let result = part1(include_str!("./test1.txt"));
        assert_eq!(result, 142);
    }
}