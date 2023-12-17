use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take},
    character::complete::{alpha1, newline},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};
use num_integer::lcm;

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let (_, (directions, nodes)) = parse_map(input).unwrap();

    dbg!(&directions, &nodes);

    let mut steps = Vec::<u64>::new();
    let current_nodes = nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .copied()
        .collect::<Vec<&str>>();

    dbg!(&current_nodes);

    for &current_node in &current_nodes {
        let mut next_node = current_node;
        let mut current_steps = 0_u64;

        while !next_node.ends_with('Z') {
            let direction = directions[current_steps as usize % directions.len()];

            next_node = match direction {
                'L' => nodes.get(next_node).unwrap().left,
                'R' => nodes.get(next_node).unwrap().right,
                value => panic!("expected L or R, got {value}"),
            };

            current_steps += 1;
        }

        steps.push(current_steps);
    }

    dbg!(&steps);

    calculate_lcm(&steps).to_string()
}

fn calculate_lcm(numbers: &[u64]) -> u64 {
    numbers.iter().copied().fold(1, lcm)
}

// fn at_end(current_nodes: &[&str]) -> bool {
//     current_nodes.len()
//         == current_nodes
//             .iter()
//             .filter(|node| node.ends_with('Z'))
//             .count()
// }

fn parse_map(input: &str) -> IResult<&str, (Vec<char>, HashMap<&str, Node>)> {
    let (input, directions) = terminated(alpha1, tag("\n\n"))(input)?;

    dbg!(directions, input);

    let (input, node_definitions) = many1(parse_node)(input)?;

    dbg!(&node_definitions, input);

    Ok((
        input,
        (
            directions.chars().collect(),
            node_definitions.into_iter().collect(),
        ),
    ))

    // todo!()
}

fn parse_node(input: &str) -> IResult<&str, (&str, Node)> {
    let (input, key) = take(3usize)(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = take(3usize)(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = take(3usize)(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = opt(newline)(input)?;

    Ok((
        input,
        (
            key,
            Node {
                left,
                right,
            },
        ),
    ))

    //todo!()
}

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );

        assert_eq!(result, "6");
    }
}
