use nom::branch::alt;
use nom::character::complete::u32;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, IResult};

fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut games = Vec::<Game>::new();

    input.lines().for_each(|line| {
        let (_, game) = parse_game(line).unwrap();
        games.push(game);
    });

    dbg!(&games);
    dbg!(&games.len());

    games
        .iter()
        .fold(0, |acc, game| {
            acc + game.max_red * game.max_green * game.max_blue
        })
        .to_string()
}

#[allow(dead_code)]
#[derive(Debug)]
struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = u32(input)?;
    dbg!(id);
    dbg!(input);

    let (input, _) = tag(": ")(input)?;

    dbg!(input);

    let (input, sets) = separated_list0(
        tag("; "),
        separated_list0(
            tag(", "),
            separated_pair(u32, tag(" "), alt((tag("red"), tag("green"), tag("blue")))),
        ),
    )(input)?;

    dbg!(&sets);
    dbg!(input);

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    sets.into_iter().for_each(|set| {
        set.into_iter().for_each(|(count, color)| match color {
            "red" => {
                if count > max_red {
                    max_red = count;
                }
            }
            "green" => {
                if count > max_green {
                    max_green = count;
                }
            }
            "blue" => {
                if count > max_blue {
                    max_blue = count;
                }
            }
            _ => (),
        })
    });

    Ok((
        input,
        Game {
            id,
            max_red,
            max_green,
            max_blue,
        },
    ))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        assert_eq!(result, "2286");
    }
}
