pub struct RingBuffer<T> {
    data: Vec<T>,
    capacity: usize,
    head: usize,
    len: usize,
}

impl<T: Default + Clone> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![T::default(); capacity],
            capacity,
            head: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        self.data[self.head] = value;
        self.head = (self.head + 1) % self.capacity;
        if self.len < self.capacity {
            self.len += 1;
        }
    }

    pub fn as_vec_newest_last(&self) -> Vec<T> {
        if self.len == 0 {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(self.len);
        let start = if self.len < self.capacity {
            0
        } else {
            self.head
        };

        for i in 0..self.len {
            let idx = (start + i) % self.capacity;
            result.push(self.data[idx].clone());
        }
        result
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl RingBuffer<f64> {
    pub fn as_sparkline_data(&self) -> Vec<u64> {
        self.as_vec_newest_last()
            .iter()
            .map(|v| *v as u64)
            .collect()
    }
}
