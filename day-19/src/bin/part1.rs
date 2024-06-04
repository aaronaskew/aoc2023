use std::collections::HashMap;

use nom::character::complete::anychar;
use nom::character::complete::digit1;
use nom::character::complete::multispace1;
use nom::character::complete::newline;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    multi::separated_list1,
    sequence::delimited,
    IResult,
};
fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn process(&self, part: &mut Part) {
        for rule in &self.rules {
            if let Some(condition) = rule.condition {
                let part_value = part.categories[&condition.category];
                match condition.operator {
                    '>' => {
                        if part_value > condition.value {
                            part.destination = rule.destination.clone();
                            break;
                        }
                    }
                    '<' => {
                        if part_value < condition.value {
                            part.destination = rule.destination.clone();
                            break;
                        }
                    }
                    _ => panic!("wrong operator"),
                }
            } else {
                part.destination = rule.destination.clone();
                break;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    condition: Option<Condition>,
    destination: Destination,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Destination {
    Accept,
    Reject,
    Send(String),
}

#[derive(Debug, Clone, Copy)]
struct Condition {
    category: char,
    operator: char,
    value: usize,
}

#[derive(Debug, Clone)]
struct Part {
    categories: HashMap<char, usize>,
    destination: Destination,
}

impl Part {
    fn total_rating(&self) -> usize {
        self.categories.values().sum()
    }
}

// a<2006:qkq
fn rule_test(input: &str) -> IResult<&str, Rule> {
    let (input, category) = alpha1(input)?;
    let (input, operator) = alt((
        nom::character::complete::char('>'),
        nom::character::complete::char('<'),
    ))(input)?;
    let (input, value) = digit1(input)?;
    let (input, destination) = preceded(nom::character::complete::char(':'), destination)(input)?;

    Ok((
        input,
        Rule {
            condition: Some(Condition {
                category: category.chars().next().unwrap(),
                operator,
                value: value.parse().unwrap(),
            }),
            destination,
        },
    ))
}

fn rule_no_test(input: &str) -> IResult<&str, Rule> {
    let (input, destination) = destination(input)?;
    Ok((
        input,
        Rule {
            condition: None,
            destination,
        },
    ))
}

fn destination(input: &str) -> IResult<&str, Destination> {
    let (input, destination) = alpha1(input).map(|(input, d)| {
        (
            input,
            match d {
                "A" => Destination::Accept,
                "R" => Destination::Reject,
                send => Destination::Send(send.to_string()),
            },
        )
    })?;
    Ok((input, destination))
}

fn rule(input: &str) -> IResult<&str, Rule> {
    alt((rule_test, rule_no_test))(input)
}

// px{a<2006:qkq,m>2090:A,rfg}
fn workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = alpha1(input)?;
    let (input, rules) = delimited(tag("{"), separated_list1(tag(","), rule), tag("}"))(input)?;
    Ok((
        input,
        Workflow {
            name: name.to_string(),
            rules,
        },
    ))
}

// {x=787,m=2655,a=1222,s=2876}
fn part(input: &str) -> IResult<&str, Part> {
    let (input, category_pairs) = delimited(
        tag("{"),
        separated_list1(tag(","), separated_pair(anychar, tag("="), digit1)),
        tag("}"),
    )(input)?;

    let categories = category_pairs
        .iter()
        .map(|(category, value)| (*category, value.parse().unwrap()))
        .collect();

    Ok((
        input,
        Part {
            categories,
            destination: Destination::Send(String::from("in")),
        },
    ))
}

fn parse(input: &str) -> IResult<&str, (HashMap<String, Workflow>, Vec<Part>)> {
    let (input, workflows) =
        separated_list1(newline, workflow)(input).map(|(input, workflows)| {
            (
                input,
                workflows
                    .iter()
                    .map(|w| (w.name.clone(), w.clone()))
                    .collect(),
            )
        })?;

    let (input, parts) = preceded(multispace1, separated_list1(multispace1, part))(input)?;

    Ok((input, (workflows, parts)))
}

fn process(input: &str) -> String {
    let (_, (workflows, mut parts)) = parse(input).unwrap();


    loop {
        if parts
            .clone()
            .iter()
            .filter(|p| matches!(p.destination, Destination::Send(_)))
            .count()
            == 0
        {
            break;
        }

        parts.iter_mut().for_each(|part| match &part.destination {
            Destination::Accept => {}
            Destination::Reject => {}
            Destination::Send(workflow) => {
                workflows[workflow].process(part);
            }
        });
    }

    // for part in parts.clone() {
    //     dbg!(&part, part.total_rating());
    // }

    parts
        .iter()
        .filter(|p| p.destination == Destination::Accept)
        .map(|p| p.total_rating())
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );

        assert_eq!(result, "19114");
    }
}
