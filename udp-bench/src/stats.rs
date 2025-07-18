use std::time::{Duration, Instant};

pub struct Stats {
    start: Instant,
    packets: u64,
    bytes: u64,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            start: Instant::now(),
            packets: 0,
            bytes: 0,
        }
    }

    pub fn record(&mut self, packet_size: usize) {
        self.packets += 1;
        self.bytes += packet_size as u64;
    }

    pub fn report(&self) {
        let elapsed = self.start.elapsed();
        let seconds = elapsed.as_secs_f64();
        let mbps = (self.bytes as f64 * 8.0) / (seconds * 1_000_000.0);
        println!(
            "Duration: {:.2}s | Packets: {} | Bytes: {} | Throughput: {:.2} Mbps",
            seconds, self.packets, self.bytes, mbps
        );
    }
}
