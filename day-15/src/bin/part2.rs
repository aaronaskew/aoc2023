use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

struct Sequence {
    steps: Vec<String>,
}

impl Sequence {
    fn _hash(&self) -> usize {
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

fn hash(s: String) -> usize {
    s.chars().map(|c| c as usize).fold(0, |acc, ascii| {
        let curr_val = acc;
        let curr_val = curr_val + ascii;
        let curr_val = curr_val * 17;
        curr_val % 256
    })
}

#[derive(Debug)]
struct LensBox {
    lenses: Vec<Lens>,
}

impl LensBox {
    fn new() -> Self {
        Self { lenses: Vec::new() }
    }

    fn lens_index(&self, label: &String) -> Option<usize> {
        for (i, lens) in self.lenses.iter().enumerate() {
            if lens.label == *label {
                return Some(i);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Lens {
    focal_length: usize,
    label: String,
}

#[derive(Debug)]
enum Command {
    Remove,
    Insert(usize),
}

fn process(input: &str) -> String {
    let steps = input.split(',').map(|s| s.to_string()).collect();
    let sequence = Sequence { steps };

    let mut boxes: HashMap<usize, LensBox> = (0..256).map(|i| (i, LensBox::new())).collect();

    for step in sequence.steps {
        dbg!(&step);
        let label = step.split(&['=', '-'][..]).next().unwrap().to_string();
        dbg!(&label);
        let box_num = hash(label.clone());
        dbg!(&box_num);
        let command = if step.contains('-') {
            Command::Remove
        } else {
            Command::Insert(step.chars().last().unwrap().to_digit(10).unwrap() as usize)
        };
        dbg!(&command);

        boxes.entry(box_num).and_modify(|b| match command {
            Command::Remove => {
                if let Some(lens_index) = b.lens_index(&label) {
                    b.lenses.remove(lens_index);
                }
            }
            Command::Insert(focal_length) => {
                let new_lens = Lens {
                    focal_length,
                    label: label.clone(),
                };
                if let Some(lens_index) = b.lens_index(&label) {
                    b.lenses[lens_index] = new_lens;
                } else {
                    b.lenses.push(new_lens);
                }
            }
        });

        // dbg!(&boxes);
    }

    boxes
        .iter()
        .map(|(box_num, lens_box)| {
            lens_box
                .lenses
                .iter()
                .enumerate()
                .map(|(slot, lens)| (box_num + 1) * (slot + 1) * lens.focal_length)
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

        assert_eq!(result, "145");
    }
}
