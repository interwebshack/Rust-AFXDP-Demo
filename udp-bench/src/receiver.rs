use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

/// Stats design rationale:
/// ------------------------
/// 1. We avoid using Mutex or any locks in the hot packet reception path to prevent contention
///    and latency under high PPS loads.
/// 2. Atomic counters (packets and bytes) are updated in O(1) time per packet, ensuring minimal overhead.
/// 3. A background stats reporter task reads these counters periodically (every second) and computes
///    throughput and PPS, isolating reporting from packet processing.
/// 4. This design guarantees that packet reception will never block due to stats collection,
///    which is critical for AF_XDP-like performance scenarios.
///
/// For reporting or exporting detailed structured stats (e.g., JSON or Prometheus), a `Stats` struct
/// is introduced as a pure aggregator. It does NOT introduce locking in the data path and simply
/// serves to format metrics for display or future integrations.

pub struct Stats {
    pub elapsed_secs: f64,
    pub packets: u64,
    pub bytes: u64,
    pub mbps: f64,
}

impl Stats {
    pub fn new(elapsed_secs: f64, packets: u64, bytes: u64) -> Self {
        let mbps = (bytes as f64 * 8.0) / (elapsed_secs * 1_000_000.0);
        Stats {
            elapsed_secs,
            packets,
            bytes,
            mbps,
        }
    }

    pub fn display(&self) {
        println!(
            "[Stats] Elapsed: {:.0}s | Packets: {} | Bytes: {} | Throughput: {:.2} Mbps",
            self.elapsed_secs, self.packets, self.bytes, self.mbps
        );
    }
}

pub async fn run(port: u16, buffer_size: usize) -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let socket = Arc::new(UdpSocket::bind(&addr).await?);
    println!(
        "Listening for UDP packets on {} with buffer size {}",
        addr, buffer_size
    );

    let packet_counter = Arc::new(AtomicU64::new(0));
    let byte_counter = Arc::new(AtomicU64::new(0));
    let start = Instant::now();

    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(buffer_size);

    // Stats reporter
    {
        let packet_counter = Arc::clone(&packet_counter);
        let byte_counter = Arc::clone(&byte_counter);
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(1)).await;
                let elapsed = start.elapsed().as_secs_f64();
                if elapsed > 0.0 {
                    let packets = packet_counter.load(Ordering::Relaxed);
                    let bytes = byte_counter.load(Ordering::Relaxed);
                    let stats = Stats::new(elapsed, packets, bytes);
                    stats.display();
                }
            }
        });
    }

    // Receiver task: receive and enqueue
    tokio::spawn({
        let socket  = Arc::clone(&socket);
        let packet_counter = Arc::clone(&packet_counter);
        let byte_counter = Arc::clone(&byte_counter);
        async move {
            let mut buf = vec![0u8; 65535];
            loop {
                match socket.recv(&mut buf).await {
                    Ok(size) => {
                        packet_counter.fetch_add(1, Ordering::Relaxed);
                        byte_counter.fetch_add(size as u64, Ordering::Relaxed);

                        let packet = buf[..size].to_vec();
                        if tx.send(packet).await.is_err() {
                            eprintln!("Worker channel closed");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Receive error: {}", e);
                        break;
                    }
                }
            }
        }
    });

    // Dummy worker to consume packets
    while let Some(_packet) = rx.recv().await {
        // Future: Integrate AF_XDP or custom processing here
        tokio::task::yield_now().await;
    }

    Ok(())
}
