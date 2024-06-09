use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::opt,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult, Parser,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Module {
    id: String,
    outputs: Vec<String>,
    memory: HashMap<String, Signal>,
    module_type: ModuleType,
    on: bool,
}

impl Module {
    fn generate_pulses(&mut self, input: Pulse) -> Vec<Pulse> {
        let mut pulses = vec![];

        match self.module_type {
            ModuleType::FlipFlop => match input.signal {
                Signal::High => {}
                Signal::Low => {
                    let signal = match self.on {
                        true => Signal::Low,
                        false => Signal::High,
                    };

                    self.on = !self.on;

                    for output in &self.outputs {
                        pulses.push(Pulse::new(signal.clone(), self.id.clone(), output.clone()))
                    }
                }
            },

            ModuleType::Conjunction => {
                self.memory
                    .entry(input.from)
                    .and_modify(|signal| *signal = input.signal);

                let pulse_signal = if self
                    .memory
                    .iter()
                    .all(|(_, signal)| *signal == Signal::High)
                {
                    Signal::Low
                } else {
                    Signal::High
                };

                for output in &self.outputs {
                    pulses.push(Pulse::new(
                        pulse_signal.clone(),
                        self.id.clone(),
                        output.clone(),
                    ));
                }
            }
            ModuleType::Broadcaster => {
                for output in &self.outputs {
                    pulses.push(Pulse::new(
                        input.signal.clone(),
                        self.id.clone(),
                        output.clone(),
                    ));
                }
            }
            ModuleType::Receiver => {}
        }

        pulses
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
    Receiver,
}

#[derive(Debug, Clone)]
struct Pulse {
    signal: Signal,
    from: String,
    to: String,
}

impl Pulse {
    fn new(signal: Signal, from: String, to: String) -> Self {
        Self { signal, from, to }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Signal {
    High,
    Low,
}

fn module(input: &str) -> IResult<&str, Module> {
    let (input, (prefix, id)) = tuple((opt(alt((tag("&"), tag("%")))), alpha1))(input)?;

    let (input, outputs) = preceded(
        tag(" -> "),
        separated_list1(tag(", "), alpha1)
            .map(|v| v.iter().map(|output: &&str| output.to_string()).collect()),
    )(input)?;

    let module_type = match prefix {
        Some("%") => ModuleType::FlipFlop,
        Some("&") => ModuleType::Conjunction,
        _ => ModuleType::Broadcaster,
    };

    Ok((
        input,
        Module {
            id: id.to_string(),
            outputs,
            memory: HashMap::new(),
            module_type,
            on: false,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Module>> {
    separated_list1(newline, module)(input)
}

fn initialize_conjunctions(modules: &mut HashMap<String, Module>) {
    let modules_copy = modules.clone();

    modules
        .iter_mut()
        .filter(|(_, m)| m.module_type == ModuleType::Conjunction)
        .for_each(|(conj_id, conj_module)| {
            // find all modules that have this id in their outputs.
            modules_copy
                .iter()
                .filter(|(_, m)| m.outputs.contains(conj_id))
                .for_each(|(input_id, _)| {
                    conj_module.memory.insert(input_id.clone(), Signal::Low);
                });
        })
}

fn press_button(count: usize, modules: &mut HashMap<String, Module>) -> HashMap<Signal, usize> {
    let mut pulse_tracker: HashMap<Signal, usize> =
        [(Signal::High, 0), (Signal::Low, 0)].into_iter().collect();

    for i in 0..count {
        let mut pulse_queue = VecDeque::new();

        pulse_queue.extend(
            modules
                .get_mut("broadcaster")
                .unwrap()
                .generate_pulses(Pulse::new(
                    Signal::Low,
                    String::from("button"),
                    String::from("broadcast"),
                )),
        );

        dbg!(&pulse_queue);

        // add the pulse from the button
        pulse_tracker
            .entry(Signal::Low)
            .and_modify(|count| *count += 1);

        while let Some(pulse) = pulse_queue.pop_front() {
            pulse_tracker
                .entry(pulse.signal.clone())
                .and_modify(|count| *count += 1);

            pulse_queue.extend(
                modules
                    .get_mut(&pulse.to)
                    .expect("module should exist")
                    .generate_pulses(pulse.clone()),
            );
        }
    }

    pulse_tracker
}

fn scan_for_receivers(modules: &mut HashMap<String, Module>) {
    let receiver_ids: Vec<String> = modules
        .values()
        .flat_map(|m| m.outputs.clone())
        .inspect(|s| {
            dbg!(&s);
        })
        .filter(|id| !modules.contains_key(id))
        .collect();

    dbg!(&receiver_ids);

    receiver_ids.iter().for_each(|id| {
        modules.insert(
            id.clone(),
            Module {
                id: id.clone(),
                outputs: vec![],
                memory: HashMap::new(),
                module_type: ModuleType::Receiver,
                on: false,
            },
        );
    });
}

fn process(input: &str) -> String {
    let (input, modules) = parse(input).expect("should parse");

    dbg!(&input, &modules);

    let mut modules: HashMap<String, Module> = modules
        .into_iter()
        .map(|v| (v.id.clone(), v.clone()))
        .collect();

    initialize_conjunctions(&mut modules);

    scan_for_receivers(&mut modules);

    dbg!(&modules);

    let results = press_button(1000, &mut modules);

    dbg!(&results);

    results.values().product::<usize>().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        );

        assert_eq!(result, "32000000");
    }

    #[test]
    fn example2() {
        let result = process(
            "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        );

        assert_eq!(result, "11687500");
    }
}
