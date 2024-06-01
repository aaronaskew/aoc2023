fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process("");

        assert_eq!(result, "");
    }
}
