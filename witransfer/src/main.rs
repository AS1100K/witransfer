use clap::{Parser, Subcommand};
use witransfer::discover;

#[derive(Parser)]
#[command(name = "WiTransfer")]
#[command(about = "A command-line tool for transferring files over Wi-Fi.")]
#[command(version, long_about = None)]
#[command(propagate_version  = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Discover other devices on the same network
    Discover {
        /// Address on which you will be receiving
        #[arg(default_value_t = ("0.0.0.0".to_string()))]
        receiving_addr: String,
        /// Port on which you will be receiving
        #[arg(default_value_t = 99)]
        port: u16,
    },
    Send {
        path: std::path::PathBuf
    }
}
fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Discover{port, receiving_addr} => {
            println!("Discovering... {receiving_addr}:{port}");
            discover(receiving_addr, *port)
        },
        _ => println!("Doing something else.")
    }
}
