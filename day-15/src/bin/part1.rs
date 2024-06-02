fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

struct Sequence {
    steps: Vec<String>,
}

impl Sequence {
    fn hash(&self) -> usize {
        self.steps
            .iter()
            .map(|s| {
                // Determine the ASCII code for the current character of the string.
                // Increase the current value by the ASCII code you just determined.
                // Set the current value to itself multiplied by 17.
                // Set the current value to the remainder of dividing itself by 256.

                s.chars().map(|c| c as usize).fold(0, |acc, ascii| {
                    let curr_val = acc;
                    let curr_val = curr_val + ascii;
                    let curr_val = curr_val * 17;
                    curr_val % 256
                })
            })
            .sum()
    }
}

fn process(input: &str) -> String {
    let steps = input.split(',').map(|s| s.to_string()).collect();
    let sequence = Sequence { steps };
    sequence.hash().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

        assert_eq!(result, "1320");
    }
}
