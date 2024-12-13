use crate::day13::claw_machine::ClawMachine;
use regex::Regex;
use std::str::FromStr;

pub struct ClawMachines {
    machines: Vec<ClawMachine>,
}

impl ClawMachines {
    pub fn parse(string: &str) -> ClawMachines {
        let a_regex = Regex::new(r"Button A: X\+(.+), Y\+(.+)").unwrap();
        let b_regex = Regex::new(r"Button B: X\+(.+), Y\+(.+)").unwrap();
        let prize_regex = Regex::new(r"Prize: X=(.+), Y=(.+)").unwrap();
        let a_vectors = parse_captures(&a_regex, string);
        let b_vectors = parse_captures(&b_regex, string);
        let prize_vectors = parse_captures(&prize_regex, string);
        let machines = (0..a_vectors.len())
            .map(|i| ClawMachine {
                button_a_vector: a_vectors[i],
                button_b_vector: b_vectors[i],
                prize_location: prize_vectors[i],
            })
            .collect();
        ClawMachines { machines }
    }
    pub fn sum_min_tokens(&self) -> usize {
        self.machines
            .iter()
            .map(|machine| machine.min_tokens_to_win())
            .sum()
    }
}

fn parse_captures(regex: &Regex, string: &str) -> Vec<[usize; 2]> {
    regex
        .captures_iter(string)
        .map(|capture| {
            [
                usize::from_str(capture.get(1).unwrap().as_str()).unwrap(),
                usize::from_str(capture.get(2).unwrap().as_str()).unwrap(),
            ]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day13::claw_machines::ClawMachines;
    use crate::input::input_to_string;

    #[test]
    fn can_sum_tokens_for_example() {
        let string = input_to_string("day13/example.txt").unwrap();
        let machines = ClawMachines::parse(string.as_str());
        assert_eq!(machines.sum_min_tokens(), 480)
    }
}
