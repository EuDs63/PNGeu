use crate::chunk_type::ChunkType;
use crc::{Crc,CRC_32_ISO_HDLC};
use crate::{Error, Result};
use std::{fmt, io::{BufReader, Read}};

#[derive(Debug)]
pub struct Chunk{
        length : u32,
        chunk_type : ChunkType,
        chunk_data : Vec<u8>,
        crc : u32 
}

impl Chunk {
    /// 新建一个crc实例
    const CRC_32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

    /// A 4-byte CRC (Cyclic Redundancy Check) calculated on the 
    /// chunk type code and chunk data fields
    fn crc_checksum(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        let bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        Self::CRC_32.checksum(&bytes)
    }

    pub fn new(chunk_type:ChunkType,data: Vec<u8>) -> Chunk{
        let crc = Self::crc_checksum(&chunk_type, &data);
        Chunk { length: data.len() as u32, chunk_type: chunk_type, chunk_data: data, crc: crc }
    }

    fn length(&self) -> u32{
        self.length
    }
    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }
   
    fn crc(&self) -> u32{
        self.crc
    }

    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String> {
        let data = self.chunk_data.clone();
        let data_string = String::from_utf8(data).unwrap();
        Ok(data_string)
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
       self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data().iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied() // 浅拷贝
            .collect()
    }

    pub fn read_chunk(reader: &mut BufReader<&[u8]>) -> Result<Chunk>{
        let mut buffer = [0;4];
        reader.read_exact(&mut buffer)?;
        let length = u32::from_be_bytes(buffer);

        reader.read_exact(&mut buffer)?;
        let chunk_type = buffer.try_into()?;

        let mut data = vec![0;length as usize];
        reader.read_exact(&mut data)?;
        
        reader.read_exact(&mut buffer)?;
        let crc = u32::from_be_bytes(buffer);

        if crc != Self::crc_checksum(&chunk_type, &data){
            return Err("invalid chunk".into());
        }

        Ok(Chunk { length: length, chunk_type: chunk_type, chunk_data: data, crc: crc })
        
    }
    
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        // 读取data_length
        let data_length_bytes = &value[0..4];
        let data_length = u32::from_be_bytes(data_length_bytes.try_into()?);

        // 读取chunk_type  
        let chunk_type_code = &value[4..8];
        let chunk_type_array : [u8;4] = chunk_type_code.try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type_array).unwrap();
        // 读取message_bytes
        let message_bytes = &value[8..(8 + data_length) as usize];
        let message_vec = message_bytes.to_vec();
        // 读取crc
        let crc_bytes = &value[(8 + data_length as usize)..]; 
        let read_crc = u32::from_be_bytes(crc_bytes.try_into().unwrap());

        // 验证crc是否正确
        let cal_crc = Self::crc_checksum(&chunk_type, &message_vec);  
        if read_crc == cal_crc{
             Ok(Chunk { length: data_length, chunk_type: ChunkType::try_from(chunk_type).unwrap(), chunk_data: message_vec, crc: cal_crc })
        }
        else{
            Err("invalid chunk".into())
        }
        // if let read_crc= cal_crc  {
        //     Ok(Chunk { length: data_length, chunk_type: ChunkType::try_from(chunk_type).unwrap(), chunk_data: message_vec, crc: cal_crc })
        // }else{
        //     Err("invalid chunk".into())
        // }

    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}
