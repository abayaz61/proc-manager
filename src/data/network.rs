pub struct NetworkData {
    pub bytes_received: u64,
    pub bytes_transmitted: u64,
    prev_received: u64,
    prev_transmitted: u64,
}

impl NetworkData {
    pub fn new() -> Self {
        Self {
            bytes_received: 0,
            bytes_transmitted: 0,
            prev_received: 0,
            prev_transmitted: 0,
        }
    }

    pub fn update(&mut self, total_received: u64, total_transmitted: u64) {
        self.bytes_received = total_received.saturating_sub(self.prev_received);
        self.bytes_transmitted = total_transmitted.saturating_sub(self.prev_transmitted);
        self.prev_received = total_received;
        self.prev_transmitted = total_transmitted;
    }
}
