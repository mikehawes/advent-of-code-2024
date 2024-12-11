pub struct Stones {}

impl Stones {
    pub fn parse(_: &str) -> Stones {
        Stones {}
    }
    pub fn blink_times(&mut self, _: usize) -> &Stones {
        self
    }
    pub fn count_stones(&self) -> usize {
        0
    }
}
