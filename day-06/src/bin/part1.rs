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
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let (_, mut races) = parse_races(input).unwrap();

    dbg!(&races);

    races.iter_mut().for_each(|race| race.solve());

    dbg!(&races);

    races.iter().fold(1, |acc, race| {
        acc * race.solution_velocities.len()
    }).to_string()

    //todo!();
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, _) = tag("Time: ")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = terminated(separated_list1(space1, u32), newline)(input)?;
    let (input, _) = tag("Distance: ")(input)?;
    let (input, _) = space1(input)?;
    let (input, distances) = separated_list1(space1, u32)(input)?;

    Ok((
        input,
        times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race {
                time,
                distance,
                solution_velocities: vec![],
            })
            .collect(),
    ))
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
    solution_velocities: Vec<u32>,
}

impl Race {
    fn solve(&mut self) {
        for t in 0..=self.time {
            let velocity = t;
            let distance = velocity * (self.time - t);
            if distance > self.distance {
                self.solution_velocities.push(velocity);
            }
        }

        dbg!(&self.solution_velocities.len());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200",
        );

        assert_eq!(result, "288");
    }
}
