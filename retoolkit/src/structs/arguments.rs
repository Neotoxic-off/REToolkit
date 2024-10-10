use clap::Parser;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[arg(short, long)]
    pub directory: String
}
