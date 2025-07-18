use clap::{Parser, Subcommand};

mod sender;
mod receiver;
mod stats;

#[derive(Parser)]
#[command(name = "udp-bench", about = "Rust-based UDP benchmarking tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send UDP packets to a target
    Send {
        #[arg(long)]
        target: String,
        #[arg(long)]
        port: u16,
        #[arg(long, default_value = "1400")]
        size: usize,
        #[arg(long, default_value = "10000")]
        rate: u64,
        #[arg(long, default_value = "30")]
        duration: u64,
        #[arg(long, default_value = "4")]
        concurrency: usize,
        #[arg(long, default_value_t = false)]
        random_payload: bool,
    },
    /// Receive UDP packets and report statistics
    Receive {
        #[arg(long)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Send {
            target,
            port,
            size,
            rate,
            duration,
            concurrency,
            random_payload,
        } => {
            sender::run(target, port, size, rate, duration, concurrency, random_payload).await?;
        }
        Commands::Receive { port } => {
            receiver::run(port).await?;
        }
    }
    Ok(())
}
