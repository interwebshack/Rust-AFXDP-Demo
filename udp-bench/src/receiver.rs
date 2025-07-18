use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};

pub async fn run(port: u16) -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening for UDP packets on {}", addr);

    let counter = Arc::new(AtomicU64::new(0));
    let start = Instant::now();

    // Stats reporting task
    let stats_counter = counter.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(1)).await;
            let elapsed = start.elapsed().as_secs();
            if elapsed > 0 {
                let count = stats_counter.load(Ordering::Relaxed);
                let pps = count / elapsed;
                println!(
                    "Elapsed: {}s | Packets: {} | Avg PPS: {}",
                    elapsed, count, pps
                );
            }
        }
    });

    let mut buf = vec![0u8; 65535];
    loop {
        match socket.recv(&mut buf).await {
            Ok(_) => {
                counter.fetch_add(1, Ordering::Relaxed);
            }
            Err(e) => {
                eprintln!("Receive error: {}", e);
                break;
            }
        }
    }

    Ok(())
}
