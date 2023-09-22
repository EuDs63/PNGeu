use std::str::FromStr;
use std::convert::TryFrom;
use std::fmt;
use crate::{Error,Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    code : [u8;4],
} 

impl  ChunkType {
    fn is_valid_byte(b:u8) -> bool {
        (65 <= b && b <= 90) || (97 <=b && b <= 122)
    }

    // Is the nth bit (from the right, *counting from 0*) zero?
    fn is_zero_bit(bit:u8,n:u8) -> bool {
        let mask = 1 << n;
        bit & mask == 0
    }
    pub fn bytes(&self) -> [u8;4]{
        self.code
    }
    
    fn is_valid(&self) -> bool{
       self.is_reserved_bit_valid() 
    }
    // first byte : uppercase -> critical
    fn is_critical(&self) -> bool{
        Self::is_zero_bit(self.code[0], 5)
    }
    // second byte : uppercase -> public
    fn is_public(&self) -> bool{
        Self::is_zero_bit(self.code[1], 5)
    }
    // third byte : uppercase
    fn is_reserved_bit_valid(&self) -> bool{
        Self::is_zero_bit(self.code[2], 5)
    }
    // fourth byte : uppercase -> unsafe
    fn is_safe_to_copy(&self) -> bool{
        !Self::is_zero_bit(self.code[3], 5)
    }
}

#[derive(Debug)]
pub enum ChunkTypeDecodingError {
    BadByte(u8),

    BadLength(usize),
}
impl fmt::Display for ChunkTypeDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadByte(b) => write!(f,"invalid byte: {}",b),
            Self::BadLength(l) => write!(f,"invalid length: {}",l), 
        }
    }
    
}
impl std::error::Error for ChunkTypeDecodingError{

}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        for byte in bytes.iter(){
            if !Self::is_valid_byte(*byte){
                return Err(Box::new(ChunkTypeDecodingError::BadByte(*byte)));
            }
        }
        Ok(ChunkType { code: bytes })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // let code_vec: Vec<u8> = s.as_bytes().iter().cloned().collect();
        // let code_array = code_vec.try_into().unwrap();
        // for code_str in code_array.iter(){

        // }

        // Ok(ChunkType { code: code_array })
        if s.len() != 4{
            return Err(Box::new(ChunkTypeDecodingError::BadLength(s.len())));
        }

        let mut vec: [u8;4] = [0;4];

        for (index,byte) in s.as_bytes().iter().enumerate(){
            if Self::is_valid_byte(*byte){
                vec[index] = * byte;
            }
            else 
            {
                return Err(Box::new(ChunkTypeDecodingError::BadByte(*byte)));
            }
        }
        Ok(ChunkType { code: vec })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       for b in &self.bytes(){
        write!(f,"{}",char::from(*b))?;
       }
       Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

