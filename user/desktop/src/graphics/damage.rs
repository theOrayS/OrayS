use alloc::vec::Vec;

use super::geometry::Rect;

pub struct DamageTracker {
    regions: Vec<Rect>,
    max_regions: usize,
}

impl DamageTracker {
    pub const fn new(max_regions: usize) -> Self {
        Self {
            regions: Vec::new(),
            max_regions,
        }
    }

    pub fn add(&mut self, mut region: Rect) {
        if region.is_empty() {
            return;
        }

        let mut index = 0;
        while index < self.regions.len() {
            if self.regions[index].touches_or_intersects(region) {
                region = region.union(self.regions.remove(index));
                index = 0;
            } else {
                index += 1;
            }
        }
        self.regions.push(region);

        if self.regions.len() > self.max_regions.max(1) {
            let merged = self
                .regions
                .iter()
                .copied()
                .reduce(Rect::union)
                .unwrap_or_default();
            self.regions.clear();
            self.regions.push(merged);
        }
    }

    pub fn regions(&self) -> &[Rect] {
        &self.regions
    }

    pub fn clear(&mut self) {
        self.regions.clear();
    }
}
