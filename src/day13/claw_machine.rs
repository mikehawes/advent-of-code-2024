pub struct ClawMachine {
    button_a_vector: ButtonVector,
    button_b_vector: ButtonVector,
    prize_location: Location,
}

impl ClawMachine {
    pub(crate) fn min_a_b_presses_to_win(&self) -> [usize; 2] {
        let [a_x, a_y] = self.button_a_vector;
        let [b_x, b_y] = self.button_b_vector;
        let [prize_x, prize_y] = self.prize_location;
        for a in 0..=100 {
            let after_a = [a_x * a, a_y * a];
            let remaining = [prize_x - after_a[0], prize_y - after_a[1]];
            if remaining[0] % b_x == 0 && remaining[1] % b_y == 0 {
                let b = remaining[0] / b_x;
                return [a, b];
            }
        }
        [0, 0]
    }
}

type ButtonVector = [usize; 2];

type Location = [usize; 2];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_presses() {
        let machine = ClawMachine {
            button_a_vector: [94, 34],
            button_b_vector: [22, 67],
            prize_location: [8400, 5400],
        };
        assert_eq!(machine.min_a_b_presses_to_win(), [80, 40]);
    }
}
