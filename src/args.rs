use clap::{Parser, Subcommand};

/// simple program to let you hide secret messages in PNG files.
#[derive(Parser)]
#[command(version)]
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

