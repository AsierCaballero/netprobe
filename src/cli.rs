use clap::Parser;

#[derive(Parser)]
#[command(name = "netprobe", version)]
pub struct Cli {
    pub targets: Vec<String>,
}
