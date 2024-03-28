use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::cli_args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::Png;
use crate::Result;
use std::fs;
use std::str::FromStr;

pub fn encode(args: EncodeArgs) -> Result<()> {
    let file_contents = fs::read(&args.file_path)?;
    let mut png = Png::try_from(file_contents.as_ref())?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);
    if let Some(file) = args.output_file {
        fs::write(file, png.as_bytes())?;
    } else {
        fs::write(&args.file_path, png.as_bytes())?;
    }
    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    let file_contents = fs::read(&args.file_path)?;
    let png = Png::try_from(file_contents.as_ref())?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = png
        .chunks()
        .iter()
        .find(|chunk| chunk.chunk_type().bytes() == chunk_type.bytes());

    if let Some(c) = chunk {
        let message = c.data_as_string()?;
        println!("{}", message);
    } else {
        println!("no message decoded");
    }
    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    let file_contents = fs::read(&args.file_path)?;
    let mut png = Png::try_from(file_contents.as_ref())?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let removed = png.remove_chunk(chunk_type);
    if let Ok(_) = removed {
        fs::write(&args.file_path, png.as_bytes())?;
        println!("message removed");
    } else {
        println!("no message to remove");
    }
    Ok(())
}

pub fn print(args: PrintArgs) -> Result<()> {
    let file_contents = fs::read(&args.file_path)?;
    let png = Png::try_from(file_contents.as_ref())?;
    let chunks = png.chunks().to_vec();
    let chunks_with_message = chunks
        .split(|chunk| chunk.chunk_type().bytes().as_ref() == b"IEND")
        .last();
    if let Some(chunks) = chunks_with_message {
        for chunk in chunks {
            println!("{}", chunk.chunk_type());
        }
    }
    Ok(())
}
