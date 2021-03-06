use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about = "Server-Script!", version = "1.0.0", author = "dolphin2410")]
pub struct Cli {
    #[clap(long, default_value_t = crate::config::default_server())]
    pub server: String,

    #[clap(long)]
    pub backup: bool,

    #[clap(long)]
    pub restart: bool,

    #[clap(long, default_value_t = crate::config::memory())]
    pub memory: i32,

    #[clap(long)]
    pub no_update: bool
}

pub fn parse() -> Cli {
    Cli::parse()
}