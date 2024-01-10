use itertools::{Itertools, Position};

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut first_numbers: Vec<i64> = vec![];
            loop {
                if nums.iter().all(|num| num == &0) {
                    break;
                }
                nums = nums
                    .iter()
                    .tuple_windows::<(&i64, &i64)>()
                    .with_position()
                    .map(|(position, (left, right))| {
                        match position {
                            Position::First | Position::Only => {
                                first_numbers.push(*left);
                            }
                            _ => {}
                        };
                        right - left
                    })
                    .collect::<Vec<i64>>();
            }
            dbg!(&first_numbers);
            let it = first_numbers.iter().rev();
            dbg!(&it);
            it.fold(0, |acc, num| num - acc)
        })
        // .collect::<Vec<i64>>();
        .sum::<i64>();

    dbg!(result.to_string())

    // todo!("all");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part2(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );

        assert_eq!(result, "2");
    }
}
