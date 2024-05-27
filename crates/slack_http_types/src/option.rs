#[derive(Clone, Copy)]
pub struct Limit(u16);

impl Default for Limit {
    fn default() -> Self {
        Self(100)
    }
}

impl Limit {
    pub fn new(limit: u16) -> Option<Self> {
        if limit <= 1_000 {
            Some(Self(limit))
        } else {
            None
        }
    }

    pub fn get(&self) -> u16 {
        self.0
    }
}
