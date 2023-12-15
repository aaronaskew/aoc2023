use std::ops::RangeInclusive;

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::u64,
    character::complete::{newline, space1},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let (_, almanac) = parse_almanac(input).unwrap();

    dbg!(&almanac);

    let locations: Vec<u64> = {
        almanac
            .seed_ranges
            .iter()
            .flat_map(|range| range.clone())
            // .into_iter()
            // .progress()
            .map(|seed| {
                almanac.humidity_to_location.lookup(
                    almanac.temperature_to_humidity.lookup(
                        almanac.light_to_temperature.lookup(
                            almanac.water_to_light.lookup(
                                almanac.fertilizer_to_water.lookup(
                                    almanac
                                        .soil_to_fertilizer
                                        .lookup(almanac.seed_to_soil.lookup(seed)),
                                ),
                            ),
                        ),
                    ),
                )
            })
            .collect()
    };

    dbg!(&locations.len());

    //locations.sort();
    //dbg!(&locations);

    locations.iter().min().unwrap().to_string()

    // todo!()
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seed_ranges) = parse_seed_ranges(input)?;
    //dbg!(input, &seed_ranges);
    let (input, seed_to_soil) = parse_mapping(input)?;
    let (input, soil_to_fertilizer) = parse_mapping(input)?;
    let (input, fertilizer_to_water) = parse_mapping(input)?;
    let (input, water_to_light) = parse_mapping(input)?;
    let (input, light_to_temperature) = parse_mapping(input)?;
    let (input, temperature_to_humidity) = parse_mapping(input)?;
    let (input, humidity_to_location) = parse_mapping(input)?;

    Ok((
        input,
        Almanac {
            seed_ranges,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    ))
}

fn parse_seed_ranges(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = terminated(separated_list1(space1, u64), tag("\n\n"))(input)?;

    let mut seed_ranges = Vec::<RangeInclusive<u64>>::new();

    let mut i = 0_usize;
    while i < seeds.len() {
        seed_ranges.push(seeds[i]..=seeds[i] + seeds[i + 1] - 1);
        i += 2;
    }

    Ok((input, seed_ranges))
}

fn parse_value_line(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, (dest, _, source, _, count, _)) =
        tuple((u64, space1, u64, space1, u64, opt(newline)))(input)?;
    Ok((input, vec![dest, source, count]))
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, _) = take_till(|c: char| c.is_ascii_digit())(input)?;
    //dbg!(input);

    let (input, values) = many1(parse_value_line)(input)?;

    //dbg!(&values);

    let destination = values
        .iter()
        .map(|v| v[0]..=v[0] + v[2] - 1)
        .collect::<Vec<RangeInclusive<u64>>>();
    let source = values
        .iter()
        .map(|v| v[1]..=v[1] + v[2] - 1)
        .collect::<Vec<RangeInclusive<u64>>>();

    Ok((
        input,
        Mapping {
            source,
            destination,
        },
    ))
}

#[derive(Debug)]
struct Almanac {
    seed_ranges: Vec<RangeInclusive<u64>>,
    seed_to_soil: Mapping,
    soil_to_fertilizer: Mapping,
    fertilizer_to_water: Mapping,
    water_to_light: Mapping,
    light_to_temperature: Mapping,
    temperature_to_humidity: Mapping,
    humidity_to_location: Mapping,
}

#[derive(Debug)]
struct Mapping {
    source: Vec<RangeInclusive<u64>>,
    destination: Vec<RangeInclusive<u64>>,
}

impl Mapping {
    fn lookup(&self, input: u64) -> u64 {
        let mut offset: i64 = 0;

        for i in 0..self.source.len() {
            if self.source[i].contains(&input) {
                offset = *self.destination[i].start() as i64 - *self.source[i].start() as i64;
                break;
            }
        }

        (input as i64 + offset) as u64
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part2(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );

        assert_eq!(result, "46");
    }
}
