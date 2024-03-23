use clap::Parser;

/// Welcome to `WiTransfer`
#[derive(Parser)]
#[command(name = "WiTransfer")]
#[command(about = "A command-line tool for transferring files over Wi-Fi.")]
#[command(version, long_about = None)]
#[command(propagate_version  = true)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
