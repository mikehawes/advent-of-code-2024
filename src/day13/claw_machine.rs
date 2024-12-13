use crate::day13::line_intersection::{find_units_along_each_line, Point};

pub struct ClawMachine {
    pub button_a_vector: ButtonVector,
    pub button_b_vector: ButtonVector,
    pub prize_location: Point,
}

impl ClawMachine {
    pub fn min_tokens_to_win(&self) -> usize {
        let a_first = self
            .find_presses(self.button_a_vector, self.button_b_vector)
            .map(|[a, b]| a * 3 + b);
        let b_first = self
            .find_presses(self.button_b_vector, self.button_a_vector)
            .map(|[b, a]| a * 3 + b);
        [a_first, b_first]
            .iter()
            .flatten()
            .min()
            .copied()
            .unwrap_or(0)
    }
    fn find_presses(
        &self,
        from_origin: ButtonVector,
        to_prize: ButtonVector,
    ) -> Option<[usize; 2]> {
        let prize = self.prize_location;
        let line_1 = [[0, 0], from_origin];
        let line_2 = [prize, [prize[0] - to_prize[0], prize[1] - to_prize[1]]];
        find_units_along_each_line(&line_1, &line_2)
    }
}

pub type ButtonVector = [usize; 2];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_min_tokens() {
        let machine = ClawMachine {
            button_a_vector: [94, 34],
            button_b_vector: [22, 67],
            prize_location: [8400, 5400],
        };
        assert_eq!(machine.min_tokens_to_win(), 280);
    }
}
