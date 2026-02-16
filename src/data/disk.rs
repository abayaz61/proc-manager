#[allow(dead_code)]
pub struct DiskData {
    pub total_space: u64,
    pub used_space: u64,
}

impl DiskData {
    pub fn new() -> Self {
        Self {
            total_space: 0,
            used_space: 0,
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, total: u64, available: u64) {
        self.total_space = total;
        self.used_space = total.saturating_sub(available);
    }
}
