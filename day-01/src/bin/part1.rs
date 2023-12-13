fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut left_digit = 0;
            let mut right_digit = 0;

            for c in line.chars() {
                if c.is_ascii_digit() {
                    left_digit = c.to_digit(10).unwrap();
                    dbg!(left_digit);
                    break;
                }
            }

            for c in line.chars().rev() {
                if c.is_ascii_digit() {
                    right_digit = c.to_digit(10).unwrap();
                    dbg!(right_digit);
                    break;
                }
            }
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
        let result = part1(
            "1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet",
        );

        assert_eq!(result, "142");
    }
}
