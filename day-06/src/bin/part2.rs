use indicatif::ProgressIterator;
use nom::{
    bytes::complete::tag,
    character::complete::space1,
    character::complete::{newline, u32},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let (_, mut race) = parse_race(input).unwrap();

    dbg!(&race);

    race.solve();

    // dbg!(&race);

    race.solution_velocities.len().to_string()

    //todo!();
}

fn parse_race(input: &str) -> IResult<&str, Race> {
    let (input, _) = tag("Time: ")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = terminated(separated_list1(space1, u32), newline)(input)?;
    let (input, _) = tag("Distance: ")(input)?;
    let (input, _) = space1(input)?;
    let (input, distances) = separated_list1(space1, u32)(input)?;

    let time = times
        .iter()
        .fold("".to_string(), |acc, t| format!("{}{}", acc, t))
        .parse::<u64>()
        .unwrap();
    let distance = distances
        .iter()
        .fold("".to_string(), |acc, d| format!("{}{}", acc, d))
        .parse::<u64>()
        .unwrap();

    Ok((
        input,
        Race {
            time,
            distance,
            solution_velocities: vec![],
        },
    ))
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
    solution_velocities: Vec<u64>,
}

impl Race {
    fn solve(&mut self) {
        let times = (0..=self.time).collect::<Vec<_>>();

        times.iter().progress().for_each(|t| {
            let velocity = t;
            let distance = velocity * (self.time - t);
            if distance > self.distance {
                self.solution_velocities.push(*velocity);
            }
        });

        dbg!(&self.solution_velocities.len());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part2(
            "Time:      7  15   30
Distance:  9  40  200",
        );

        assert_eq!(result, "71503");
    }
}
