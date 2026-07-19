#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ElapsedClock {
    milliseconds: u64,
}

impl ElapsedClock {
    pub const fn new() -> Self {
        Self { milliseconds: 0 }
    }

    pub fn advance(&mut self, elapsed_ms: u32) {
        self.milliseconds = self.milliseconds.saturating_add(elapsed_ms as u64);
    }

    pub const fn milliseconds(&self) -> u64 {
        self.milliseconds
    }

    pub const fn whole_minutes(&self) -> u64 {
        self.milliseconds / 60_000
    }
}
