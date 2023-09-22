use clap::{Parser, Subcommand};

/// simple program to let you hide secret messages in PNG files.
#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub args: PngMeArgs,
}

#[derive(Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}
/// Encode a message into a PNG file
#[derive(Parser)]
pub struct EncodeArgs {
    pub file_path: String,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<String>,
}
/// Decode a message stored in a PNG file
#[derive(Parser)]
pub struct DecodeArgs {
    pub file_path: String,

    pub chunk_type: String,
}
/// Remove a message from a PNG file
#[derive(Parser)]
pub struct RemoveArgs {
    pub file_path: String,

    pub chunk_type: String,
}
/// Print a list of PNG chunks that can be searched for messages
#[derive(Parser)]
pub struct PrintArgs {
    pub file_path: String,
}

/// Simple program to greet a personp
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 3)]
    count: u8,
}

impl Args {
    pub fn run() {
        let args = Args::parse();

        for _ in 0..args.count {
            println!("Hello {}!", args.name)
        }
    }
}
