use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use rand::Rng;
use tokio::net::UdpSocket;
use tokio::sync::Barrier;
use tokio::time::sleep;

pub async fn run(
    target: String,
    port: u16,
    size: usize,
    rate: u64,
    duration: u64,
    concurrency: usize,
    random_payload: bool,
) -> anyhow::Result<()> {
    let addr: SocketAddr = format!("{}:{}", target, port).parse()?;

    println!(
        "Starting UDP send to {} | size={} bytes | rate={} pps | duration={} sec | concurrency={} | random_payload={}",
        addr, size, rate, duration, concurrency, random_payload
    );

    let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);
    socket.connect(addr).await?;

    let packets_per_task = rate / concurrency as u64;
    let barrier = Arc::new(Barrier::new(concurrency));

    let start = Instant::now();

    let tasks: Vec<_> = (0..concurrency)
        .map(|_| {
            let socket = Arc::clone(&socket);
            let barrier = Arc::clone(&barrier);

            // Pre-generate packet to avoid using non-Send RNG inside async
            let packet = if random_payload {
                let mut rng = rand::thread_rng();
                let mut data = vec![0u8; size];
                rng.fill(&mut data[..]);
                data
            } else {
                vec![0u8; size]
            };

            tokio::spawn(async move {
                barrier.wait().await;
                let interval = Duration::from_secs_f64(1.0 / packets_per_task as f64);
                let mut sent_packets: u64 = 0;

                while start.elapsed().as_secs() < duration {
                    let before = Instant::now();
                    if let Err(e) = socket.send(&packet).await {
                        eprintln!("Send error: {}", e);
                    }
                    sent_packets += 1;

                    let delay = interval.saturating_sub(before.elapsed());
                    if delay > Duration::ZERO {
                        sleep(delay).await;
                    }
                }
                sent_packets
            })
        })
        .collect();

    let mut total_sent = 0u64;
    for t in tasks {
        total_sent += t.await?;
    }

    let total_bytes = total_sent * size as u64;
    let mbps = (total_bytes as f64 * 8.0) / (duration as f64 * 1_000_000.0);
    println!(
        "\n--- Summary ---\nPackets sent: {}\nTotal bytes: {}\nThroughput: {:.2} Mbps",
        total_sent, total_bytes, mbps
    );

    Ok(())
}
