use crate::day13::line_intersection::Point;

pub struct ClawMachine {
    pub button_a_vector: ButtonVector,
    pub button_b_vector: ButtonVector,
    pub prize_location: Point,
}

impl ClawMachine {
    pub fn min_a_b_presses_to_win(&self) -> [usize; 2] {
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
    pub fn min_tokens_to_win(&self) -> usize {
        let [a, b] = self.min_a_b_presses_to_win();
        a * 3 + b
    }
}

pub type ButtonVector = [usize; 2];

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
        assert_eq!(machine.min_tokens_to_win(), 280);
    }
}
