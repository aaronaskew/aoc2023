use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let (_, (directions, nodes)) = parse_map(input).unwrap();

    dbg!(&directions, &nodes);

    let mut steps = 0_u32;
    let mut current_node = nodes.get("AAA").unwrap();

    while current_node.label != "ZZZ" {
        let direction = directions[steps as usize % directions.len()];

        current_node = nodes
            .get(match direction {
                'L' => current_node.left,
                'R' => current_node.right,
                value => panic!("expected L or R, found {value}"),
            })
            .unwrap();

        steps += 1;
    }

    steps.to_string()

    //todo!();
}



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
    let (input, key) = alpha1(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = alpha1(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = alpha1(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = opt(newline)(input)?;

    Ok((
        input,
        (
            key,
            Node {
                label: key,
                left,
                right,
            },
        ),
    ))

    //todo!()
}

#[derive(Debug)]
struct Node<'a> {
    label: &'a str,
    left: &'a str,
    right: &'a str,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part1(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );

        assert_eq!(result, "2");

        let result = part1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );

        assert_eq!(result, "6");
    }
}
