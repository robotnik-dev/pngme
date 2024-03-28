use crate::{Error, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChunkType {
    pub data: [u8; 4]
}

impl std::convert::TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        let chunk_type = ChunkType{data: value};
        for byte in value.iter() {
            if !ChunkType::is_valid_byte(*byte) {
                return Err("Invalid chunk type: Not ASCII alphabetic".into());
            }
        }
        Ok(chunk_type)
    }
}

impl std::str::FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err("Invalid chunk type: Length must be 4".into());
        }
        let mut bytes = [0; 4];
        for (i, byte) in s.bytes().enumerate() {
            if !ChunkType::is_valid_byte(byte) {
                return Err("Invalid chunk type: Not ASCII alphabetic".into());
            }
            bytes[i] = byte;
        }
        Ok(ChunkType{data: bytes})
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.data).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", s)
    }
}

impl ChunkType {
    /// Returns the raw bytes contained in this chunk
    pub fn bytes(&self) -> [u8; 4] {
        self.data
    }

    /// Returns the property state of the first byte
    #[allow(dead_code)]
    pub fn is_critical(&self) -> bool {
        self.data[0].is_ascii_uppercase()
    }

    /// Returns the property state of the second byte 
    #[allow(dead_code)]
    pub fn is_public(&self) -> bool {
        self.data[1].is_ascii_uppercase()
    }

    /// Returns the property state of the third byte 
    #[allow(dead_code)]
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.data[2].is_ascii_uppercase()
    }

    /// Returns the property state of the fourth byte
    #[allow(dead_code)]
    pub fn is_safe_to_copy(&self) -> bool {
        self.data[3].is_ascii_lowercase()
    }

    /// Note that this chunk type should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool {
        true
    }

    /// Valid bytes are represented by the characters A-Z or a-z
    fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_alphabetic() && byte.is_ascii()
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
        assert!(chunk.is_valid());

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