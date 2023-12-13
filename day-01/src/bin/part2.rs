fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut left_digit = 0;
            let mut right_digit = 0;

            for index in 0..line.len() {
                let slice = &line[index..];

                dbg!(slice);

                let c = slice.chars().next().unwrap();
                if c.is_ascii_digit() {
                    left_digit = c.to_digit(10).unwrap();
                    break;
                } else if slice.starts_with("one") {
                    left_digit = 1;
                    break;
                } else if slice.starts_with("two") {
                    left_digit = 2;
                    break;
                } else if slice.starts_with("three") {
                    left_digit = 3;
                    break;
                } else if slice.starts_with("four") {
                    left_digit = 4;
                    break;
                } else if slice.starts_with("five") {
                    left_digit = 5;
                    break;
                } else if slice.starts_with("six") {
                    left_digit = 6;
                    break;
                } else if slice.starts_with("seven") {
                    left_digit = 7;
                    break;
                } else if slice.starts_with("eight") {
                    left_digit = 8;
                    break;
                } else if slice.starts_with("nine") {
                    left_digit = 9;
                    break;
                }
            }
            dbg!(left_digit);

            let mut index = line.len();
            while index > 0 {
                
            
                let slice = &line[..index];

                dbg!(slice);

                let c = slice.chars().last().unwrap();
                if c.is_ascii_digit() {
                    right_digit = c.to_digit(10).unwrap();
                    break;
                } else if slice.ends_with("one") {
                    right_digit = 1;
                    break;
                } else if slice.ends_with("two") {
                    right_digit = 2;
                    break;
                } else if slice.ends_with("three") {
                    right_digit = 3;
                    break;
                } else if slice.ends_with("four") {
                    right_digit = 4;
                    break;
                } else if slice.ends_with("five") {
                    right_digit = 5;
                    break;
                } else if slice.ends_with("six") {
                    right_digit = 6;
                    break;
                } else if slice.ends_with("seven") {
                    right_digit = 7;
                    break;
                } else if slice.ends_with("eight") {
                    right_digit = 8;
                    break;
                } else if slice.ends_with("nine") {
                    right_digit = 9;
                    break;
                }

                index -= 1;
            }
            dbg!(right_digit);

            dbg!(left_digit * 10 + right_digit);
            left_digit * 10 + right_digit
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        assert_eq!(result, "281");
    }
}
