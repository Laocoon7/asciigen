use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
pub struct Args {
    #[arg(short, long, default_value_t = 10)]
    pub width: u32,

    #[arg(short, long, default_value_t = 10)]
    pub height: u32,

    #[arg(short, long)]
    pub seed: Option<u64>,
}