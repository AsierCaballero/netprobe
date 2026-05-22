use clap::Parser;

#[derive(Parser)]
#[command(name = "netprobe", version)]
pub struct Cli {
    pub targets: Vec<String>,
    #[arg(short = 'p', long, default_value_t = 80)]
    pub port: u16,
    #[arg(short = 'n', long, default_value_t = 3)]
    pub count: u32,
}
