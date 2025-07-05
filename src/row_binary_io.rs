//! Row Binary IO utilities for parsing and serializing binary data rows.

use std::convert::TryInto;
use std::io::Read;

pub enum DataType {
    U32,
    U16,
    U8,
}

#[derive(Debug, PartialEq)]
pub enum ParsedValue {
    U32(u32),
    U16(u16),
    U8(u8),
}

pub fn parse_bytes_by_types(input: &[u8], types: &[DataType]) -> Result<Vec<ParsedValue>, String> {
    let mut result = Vec::new();
    let mut offset = 0;
    for dtype in types {
        match dtype {
            DataType::U32 => {
                if input.len() < offset + 4 {
                    return Err("Not enough bytes for u32".to_string());
                }
                let bytes: [u8; 4] = input[offset..offset + 4].try_into().unwrap();
                result.push(ParsedValue::U32(u32::from_le_bytes(bytes)));
                offset += 4;
            }
            DataType::U16 => {
                if input.len() < offset + 2 {
                    return Err("Not enough bytes for u16".to_string());
                }
                let bytes: [u8; 2] = input[offset..offset + 2].try_into().unwrap();
                result.push(ParsedValue::U16(u16::from_le_bytes(bytes)));
                offset += 2;
            }
            DataType::U8 => {
                if input.len() < offset + 1 {
                    return Err("Not enough bytes for u8".to_string());
                }
                let byte = input[offset];
                result.push(ParsedValue::U8(byte));
                offset += 1;
            }
        }
    }
    if offset != input.len() {
        return Err("Input contains extra unused bytes".to_string());
    }
    Ok(result)
}

pub fn serialize_parsed_values(values: &[ParsedValue]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for value in values {
        match value {
            ParsedValue::U32(v) => bytes.extend(&v.to_le_bytes()),
            ParsedValue::U16(v) => bytes.extend(&v.to_le_bytes()),
            ParsedValue::U8(v) => bytes.push(*v),
        }
    }
    bytes
}

pub fn read_and_parse_from_reader<R: Read>(
    mut reader: R,
    types: &[DataType],
) -> Result<Vec<ParsedValue>, String> {
    // Count the number of bytes needed for all types
    let num_bytes: usize = types
        .iter()
        .map(|t| match t {
            DataType::U32 => 4,
            DataType::U16 => 2,
            DataType::U8 => 1,
        })
        .sum();
    let mut buf = Vec::with_capacity(num_bytes);
    while buf.len() < num_bytes {
        let mut byte = [0u8; 1];
        match reader.read_exact(&mut byte) {
            Ok(_) => {
                if byte[0] != b'\n' {
                    buf.push(byte[0]);
                }
            }
            Err(e) => return Err(format!("Failed to read from reader: {}", e)),
        }
    }
    parse_bytes_by_types(&buf, types)
}

pub fn calculate_bytes_needed(types: &[DataType]) -> usize {
    types
        .iter()
        .map(|t| match t {
            DataType::U32 => 4,
            DataType::U16 => 2,
            DataType::U8 => 1,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bytes_by_types_u32_u16_u8() {
        let mut input = Vec::new();
        input.extend(&42u32.to_le_bytes());
        input.extend(&65535u16.to_le_bytes());
        input.push(255u8);
        let types = [DataType::U32, DataType::U16, DataType::U8];
        let parsed = parse_bytes_by_types(&input, &types).unwrap();
        assert!(matches!(parsed[0], ParsedValue::U32(42)));
        assert!(matches!(parsed[1], ParsedValue::U16(65535)));
        assert!(matches!(parsed[2], ParsedValue::U8(255)));
    }

    #[test]
    fn test_parse_bytes_by_types_u32_u16_u8_alternate() {
        let mut input = Vec::new();
        input.extend(&42u32.to_le_bytes());
        input.extend(&65530u16.to_le_bytes());
        input.push(7u8);
        let types = [DataType::U32, DataType::U16, DataType::U8];
        let parsed = parse_bytes_by_types(&input, &types).unwrap();
        assert!(matches!(parsed[0], ParsedValue::U32(42)));
        assert!(matches!(parsed[1], ParsedValue::U16(65530)));
        assert!(matches!(parsed[2], ParsedValue::U8(7)));
    }

    #[test]
    fn test_parse_bytes_by_types_insufficient_bytes() {
        let input = [1u8, 2u8]; // Not enough for u32
        let types = [DataType::U32];
        let result = parse_bytes_by_types(&input, &types);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_parsed_values() {
        let values = vec![
            ParsedValue::U32(42),
            ParsedValue::U16(65535),
            ParsedValue::U8(255),
        ];
        let serialized = serialize_parsed_values(&values);
        let mut expected = Vec::new();
        expected.extend(&42u32.to_le_bytes());
        expected.extend(&65535u16.to_le_bytes());
        expected.push(255u8);
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_round_trip_serialize_and_parse() {
        let original = vec![
            ParsedValue::U32(123456789),
            ParsedValue::U16(54321),
            ParsedValue::U8(200),
        ];
        let types = [DataType::U32, DataType::U16, DataType::U8];
        let bytes = serialize_parsed_values(&original);
        let parsed = parse_bytes_by_types(&bytes, &types).unwrap();
        assert_eq!(original, parsed);
    }
}
