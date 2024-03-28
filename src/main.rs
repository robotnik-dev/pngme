use clap::Parser;

mod cli_args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn main() -> Result<()> {
    let app = cli_args::App::parse();
    match app.command {
        cli_args::Commands::Encode(args) => commands::encode(args)?,
        cli_args::Commands::Decode(args) => commands::decode(args)?,
        cli_args::Commands::Remove(args) => commands::remove(args)?,
        cli_args::Commands::Print(args) => commands::print(args)?,
    }
    Ok(())
}
