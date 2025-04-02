mod client;
mod server;
mod error;

use error::Result;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    info!("Starting Iron Tunnel...");
    
    // We'll add command-line argument parsing here later
    // to determine whether to run as client or server
    
    Ok(())
}
