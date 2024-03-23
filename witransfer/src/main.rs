use clap::{Parser, Subcommand};

/// Welcome to `WiTransfer`
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
    Discover,
    Send {
        path: std::path::PathBuf
    }
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Discover => {
            println!("Discovering...")
        },
        _ => println!("Doing something else.")
    }
}
