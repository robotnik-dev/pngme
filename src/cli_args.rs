use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Encode a message in a PNG file
    Encode(EncodeArgs),
    /// Decode a message from a PNG file
    Decode(DecodeArgs),
    /// Remove a message from a PNG file
    Remove(RemoveArgs),
    /// Print a list of PNG chunks that can be searched for messages
    Print(PrintArgs),
}

#[derive(Args, Debug, Clone)]
pub struct EncodeArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,
    /// Name of the chunk to hide the message in
    pub chunk_type: String,
    /// Message to hide in the PNG file
    pub message: String,
    /// Path to the output PNG file (optional)
    pub output_file: Option<PathBuf>,
}

#[derive(Args, Debug, Clone)]
pub struct DecodeArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,
    /// Name of the chunk where the message is hidden
    pub chunk_type: String,
}

#[derive(Args, Debug, Clone)]
pub struct RemoveArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,
    /// Name of the chunk where the message is hidden and should be removed
    pub chunk_type: String,
}

#[derive(Args, Debug, Clone)]
pub struct PrintArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,
}
