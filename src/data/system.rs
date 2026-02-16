pub struct SystemData {
    pub hostname: String,
    pub uptime: u64,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub cpu_usage: Vec<f32>,
    pub global_cpu_usage: f32,
}

impl SystemData {
    pub fn new() -> Self {
        Self {
            hostname: String::new(),
            uptime: 0,
            total_memory: 0,
            used_memory: 0,
            total_swap: 0,
            used_swap: 0,
            cpu_usage: Vec::new(),
            global_cpu_usage: 0.0,
        }
    }
}
