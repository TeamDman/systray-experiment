// #![windows_subsystem = "windows"]  // Hide the console window on Windows

use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(version, about = "An experimental application")]
struct Args {
    /// If set, enable debug logging
    #[arg(long)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    
    let args = Args::parse();

    // Setup logging
    let log_level = if args.debug {
        Level::DEBUG
    } else {
        Level::INFO
    };
    let env_filter = EnvFilter::builder()
        .with_default_directive(log_level.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();



    info!("Hi!"); 
    println!("Hi!"); 
    // read line
    println!("Hit enter to close...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(())
}
