use std::{fs, str::FromStr};

use crate::{
    args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
    Result,
};

/// funtion to encode a message into ayy PNG file
pub fn encode(args: &EncodeArgs) -> Result<()>{
    let mut png: Png = fs::read(&args.file_path)?.as_slice().try_into()?;
    png.append_chunk(Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.as_bytes().into(),
    ));

    fs::write(args.output_file.as_ref().unwrap_or(&args.file_path),
                     png.as_bytes(),
             )?;
    println!("success!");
    Ok(())
}
/// funtion to dcode a message stored in a PNG file
pub fn decode(args:&DecodeArgs) -> Result<()>{
    let png: Png = fs::read(&args.file_path)?.as_slice().try_into()?;
    println!("{}",png.chunk_by_type(&args.chunk_type).unwrap().data_as_string().unwrap());
    Ok(())
}
/// function to remove a message from a PNG file
pub fn remove(args:&RemoveArgs) -> Result<()>{
   let mut png:Png = fs::read(&args.file_path)?.as_slice().try_into()?;
   match png.remove_chunk(&args.chunk_type){
        Ok(_) => println!("remove successfully!"),
        Err(e) => println!("some thing went wrong {}",e)
   }
    Ok(())
}
/// function to print a list of PNG chunks that can be searched for messages
pub fn print(args:&PrintArgs) -> Result<()>{
    let png: Png = fs::read(&args.file_path)?.as_slice().try_into()?;
    println!("{}",png.to_string());
    Ok(())
}