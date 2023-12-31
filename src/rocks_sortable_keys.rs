#![allow(dead_code)]
use std::cmp::Ordering;
use crate::rockssort_float::F32struct;
use crate::rockssort_float::F64struct; 
// use enum_dispatch::enum_dispatch;

// #[enum_dispatch]
pub trait Encode: std::fmt::Debug + Clone {
    fn encode(&self) -> Vec<u8>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncodeType {
    SortU8(u8),
    SortU16(u16),
    SortU32(u32),
    SortU64(u64),
    SortU128(u128),
    SortI32(i32),
    SortI64(i64),
    SortString(String),
    SortBytes(Vec<u8>),
    SortBool(bool),
    SortF32(F32struct),
    SortF64(F64struct),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DecodeType {
    DecodeU8,
    DecodeU16,
    DecodeU32,
    DecodeU64,
    DecodeU128,
    DecodeI32,
    DecodeI64,
    DecodeString,
    DecodeBytes,
    DecodeBool,
    DecodeF32,
    DecodeF64,
    Reverse
}

pub trait Decode: std::fmt::Debug + Clone {
    fn decode(data: &[u8], the_type: DecodeType, pos: &mut usize) -> EncodeType;
}

impl Encode for EncodeType {

#[inline]
    fn encode(&self) -> Vec<u8> {
        match self {
            EncodeType::SortU8(value) => value.to_be_bytes().to_vec(),
            EncodeType::SortU16(value) => value.to_be_bytes().to_vec(),
            EncodeType::SortU32(value) => value.to_be_bytes().to_vec(),
            EncodeType::SortU64(value) => value.to_be_bytes().to_vec(),
            EncodeType::SortU128(value) => value.to_be_bytes().to_vec(),
            EncodeType::SortI32(value) => value.to_be_bytes().to_vec(),
            EncodeType::SortI64(value) => value.to_be_bytes().to_vec(),
            EncodeType::SortString(value) => {
                let the_len = value.len() as u32;
                [the_len.to_be_bytes().to_vec(), value.as_bytes().to_vec()].concat()
            },
            EncodeType::SortBytes(value) => {
                let the_len = value.len() as u32;
                [the_len.to_be_bytes().to_vec(), value.clone()].concat()
            },
            EncodeType::SortBool(value) => {
                if *value {
                    vec![1]
                } else {
                    vec![0]
                }
            },
            EncodeType::SortF32(value) => value.value.to_be_bytes().to_vec(),
            EncodeType::SortF64(value) => value.value.to_be_bytes().to_vec(),
        }
    }
}

impl Decode for DecodeType {

#[inline]
    fn decode(data: &[u8], the_type: DecodeType, pos: &mut usize) -> EncodeType {
        match the_type {
            DecodeType::DecodeU8 => {
                let value = u8::from_be_bytes([data[*pos]]);
                *pos += 1;
                EncodeType::SortU8(value)
            },
            DecodeType::DecodeU16 => {
                let value = u16::from_be_bytes([data[*pos], data[*pos + 1]]);
                *pos += 2;
                EncodeType::SortU16(value)
            },
            DecodeType::DecodeU32 => {
                let value = u32::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]]);
                *pos += 4;
                EncodeType::SortU32(value)
            },
            DecodeType::DecodeU64 => {
                let value = u64::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3], data[*pos + 4], data[*pos + 5], data[*pos + 6], data[*pos + 7]]);
                *pos += 8;
                EncodeType::SortU64(value)
            },
            DecodeType::DecodeU128 => {
                let value = u128::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3], data[*pos + 4], data[*pos + 5], data[*pos + 6], data[*pos + 7], data[*pos + 8], data[*pos + 9], data[*pos + 10], data[*pos + 11], data[*pos + 12], data[*pos + 13], data[*pos + 14], data[*pos + 15]]);
                *pos += 16;
                EncodeType::SortU128(value)
            },
            DecodeType::DecodeI32 => {
                let value = i32::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]]);
                *pos += 4;
                EncodeType::SortI32(value)
            },
            DecodeType::DecodeI64 => {
                let value = i64::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3], data[*pos + 4], data[*pos + 5], data[*pos + 6], data[*pos + 7]]);
                *pos += 8;
                EncodeType::SortI64(value)
            },
            DecodeType::DecodeString => {
                let the_len = u32::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]]) as usize;
                *pos += 4;
                let value = String::from_utf8(data[*pos..*pos + the_len].to_vec()).unwrap();
                *pos += the_len;
                EncodeType::SortString(value)
            },
            DecodeType::DecodeBytes => {
                let the_len = u32::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]]) as usize;
                *pos += 4;
                let value = data[*pos..*pos + the_len].to_vec();
                *pos += the_len;
                EncodeType::SortBytes(value)
            },
            DecodeType::DecodeBool => {
                let value = data[*pos] == 1;
                *pos += 1;
                EncodeType::SortBool(value)
            },
            DecodeType::DecodeF32 => {
                let value = f32::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]]);
                *pos += 4;
                let f32_struct = F32struct::new(value);
                EncodeType::SortF32(f32_struct)
            },
            DecodeType::DecodeF64 => {
                let value = f64::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3], data[*pos + 4], data[*pos + 5], data[*pos + 6], data[*pos + 7]]);
                *pos += 8;
                let f64_struct = F64struct::new(value);
                EncodeType::SortF64(f64_struct)
            },
            // will never happend, just added for completeness
            DecodeType::Reverse => {
                EncodeType::SortU32(0)
            },
        }
    }
} 

#[inline]
pub fn encode_keys<T: Encode>(keys: &[T]) -> Vec<u8> {
    let mut encoded_data = Vec::new();

    for key in keys {
        encoded_data.extend(key.encode());
    }
    encoded_data
}

#[inline]
pub fn decode_byte_array(data: &[u8], the_types: &Vec<DecodeType>) -> Vec<EncodeType> {
    let mut pos = 0;
    let mut decoded_data = Vec::new();

    for the_type in the_types {
        if the_type == &DecodeType::Reverse {
            continue;
        }
        decoded_data.push(DecodeType::decode(data, the_type.clone(), &mut pos));
    }
    decoded_data
}


#[inline]
pub fn compare(the_types: &Vec<DecodeType>, key1: &[EncodeType], key2: &[EncodeType]) -> Ordering {
    let mut pos = 0;
    let mut is_reverse = false;
    for the_type in the_types {
        if the_type == &DecodeType::Reverse {
            is_reverse = true;
            continue;
        }
        let result = match the_type {
            DecodeType::DecodeU8 => {
                let key1_value = match &key1[pos] {
                    EncodeType::SortU8(value) => value,
                    _ => panic!("wrong type"),
                };
                let key2_value = match &key2[pos] {
                    EncodeType::SortU8(value) => value,
                    _ => panic!("wrong type"),
                };
                if is_reverse {
                    key2_value.cmp(key1_value)
                } else {
                    key1_value.cmp(key2_value)
                }
            },
            DecodeType::DecodeU16 => {
                let key1_value = match &key1[pos] {
                    EncodeType::SortU16(value) => value,
                    _ => panic!("wrong type"),
                };
                let key2_value = match &key2[pos] {
                    EncodeType::SortU16(value) => value,
                    _ => panic!("wrong type"),
                };
                if is_reverse {
                    key2_value.cmp(key1_value)
                } else {
                    key1_value.cmp(key2_value)
                }
            },
            DecodeType::DecodeU32 => {
                let key1_value = match &key1[pos] {
                    EncodeType::SortU32(value) => value,
                    _ => panic!("wrong type"),
                };
                let key2_value = match &key2[pos] {
                    EncodeType::SortU32(value) => value,
                    _ => panic!("wrong type"),
                };
                if is_reverse {
                    key2_value.cmp(key1_value)
                } else {
                    key1_value.cmp(key2_value)
                }
            },
            DecodeType::DecodeU64 => {
                let key1_value = match &key1[pos] {
                    EncodeType::SortU64(value) => value,
                    _ => panic!("wrong type"),
                };
                let key2_value = match &key2[pos] {
                    EncodeType::SortU64(value) => value,
                    _ => panic!("wrong type"),
                };
                if is_reverse {
                    key2_value.cmp(key1_value)
                } else {
                    key1_value.cmp(key2_value)
                }
            },
            DecodeType::DecodeU128 => {
                let key1_value = match &key1[pos] {
                    EncodeType::SortU128(value) => value,
                    _ => panic!("wrong type"),
                };
                let key2_value = match &key2[pos] {
                    EncodeType::SortU128(value) => value,
                    _ => panic!("wrong type"),
                };
                if is_reverse {
                    key2_value.cmp(key1_value)
                } else {
                    key1_value.cmp(key2_value)
                }
            },
            DecodeType::DecodeString => {
                let key1_value = match &key1[pos] {
                    EncodeType::SortString(value) => value,
                    _ => panic!("wrong type"),
                };
                let key2_value = match &key2[pos] {
                    EncodeType::SortString(value) => value,
                    _ => panic!("wrong type"),
                };
                if is_reverse {
                    key2_value.cmp(key1_value)
                } else {
                    key1_value.cmp(key2_value)
                }
            },
            DecodeType::DecodeBool => {
                let key1_value = match &key1[pos] {
                    EncodeType::SortBool(value) => value,
                    _ => panic!("wrong type"),
                };
                let key2_value = match &key2[pos] {
                    EncodeType::SortBool(value) => value,
                    _ => panic!("wrong type"),
                };
                if is_reverse {
                    key2_value.cmp(key1_value)
                } else {
                    key1_value.cmp(key2_value)
                }
            },
            DecodeType::DecodeF32 => {
                let key1_value = match &key1[pos] {
                    EncodeType::SortF32(value) => value,
                    _ => panic!("wrong type"),
                };
                let key2_value = match &key2[pos] {
                    EncodeType::SortF32(value) => value,
                    _ => panic!("wrong type"),
                };
                if is_reverse {
                    key2_value.cmp(key1_value)
                } else {
                    key1_value.cmp(key2_value)
                }
            },
            DecodeType::DecodeF64 => {
                let key1_value = match &key1[pos] {
                    EncodeType::SortF64(value) => value,
                    _ => panic!("wrong type"),
                };
                let key2_value = match &key2[pos] {
                    EncodeType::SortF64(value) => value,
                    _ => panic!("wrong type"),
                };
                if is_reverse {
                    key2_value.cmp(key1_value)
                } else {
                    key1_value.cmp(key2_value)
                }
            },
            _ => panic!("wrong type"),
        };
        if result != Ordering::Equal {
            return result;
        }
        pos += 1;
        is_reverse = false;
    }
    Ordering::Equal
}
#[inline]
pub fn compare_bytes(the_types: &Vec<DecodeType>, key1: &[u8], key2: &[u8]) -> Ordering {
    let decoded_key1 = decode_byte_array(key1, the_types);
    let decoded_key2 = decode_byte_array(key2, the_types);
    compare(the_types, &decoded_key1, &decoded_key2)
}


pub fn serialize_decode_types(the_types:&Vec<DecodeType>) -> Vec<u8> {
    let mut result = Vec::new();
    for the_type in the_types {
        match the_type {
            DecodeType::DecodeU8 => result.push(0),
            DecodeType::DecodeU16 => result.push(1),
            DecodeType::DecodeU32 => result.push(2),
            DecodeType::DecodeU64 => result.push(3),
            DecodeType::DecodeU128 => result.push(4),
            DecodeType::DecodeString => result.push(5),
            DecodeType::DecodeBool => result.push(6),
            DecodeType::DecodeF32 => result.push(7),
            DecodeType::DecodeF64 => result.push(8),
            DecodeType::Reverse => result.push(9),
            DecodeType::DecodeBytes => result.push(10),
            DecodeType::DecodeI32 => result.push(11),
            DecodeType::DecodeI64 => result.push(12),
        }
    }
    result
}

pub fn deserialize_decode_types(the_types:&Vec<u8>) -> Vec<DecodeType> {
    let mut result = Vec::new();
    for the_type in the_types {
        match the_type {
            0 => result.push(DecodeType::DecodeU8),
            1 => result.push(DecodeType::DecodeU16),
            2 => result.push(DecodeType::DecodeU32),
            3 => result.push(DecodeType::DecodeU64),
            4 => result.push(DecodeType::DecodeU128),
            5 => result.push(DecodeType::DecodeString),
            6 => result.push(DecodeType::DecodeBool),
            7 => result.push(DecodeType::DecodeF32),
            8 => result.push(DecodeType::DecodeF64),
            9 => result.push(DecodeType::Reverse),
            10 => result.push(DecodeType::DecodeBytes),
            11 => result.push(DecodeType::DecodeI32),
            12 => result.push(DecodeType::DecodeI64),
            _ => panic!("wrong type"),
        }
    }
    result
}


fn validate_types(decode_types_all: &[DecodeType], encode_types: &[EncodeType]) -> bool {
    let decode_types: Vec<DecodeType> = decode_types_all.iter().filter(|x| **x != DecodeType::Reverse).cloned().collect();
    if encode_types.len() != decode_types.len() {
        return false;
    }
    for (encode_type, decode_type) in encode_types.iter().zip(decode_types.iter()) {
        match encode_type {
            EncodeType::SortU8(_) => {
                if *decode_type != DecodeType::DecodeU8 {
                    return false;
                }
            },
            EncodeType::SortU16(_) => {
                if *decode_type != DecodeType::DecodeU16 {
                    return false;
                }
            },
            EncodeType::SortU32(_) => {
                if *decode_type != DecodeType::DecodeU32 {
                    return false;
                }
            },
            EncodeType::SortU64(_) => {
                if *decode_type != DecodeType::DecodeU64 {
                    return false;
                }
            },
            EncodeType::SortU128(_) => {
                if *decode_type != DecodeType::DecodeU128 {
                    return false;
                }
            },
            EncodeType::SortString(_) => {
                if *decode_type != DecodeType::DecodeString {
                    return false;
                }
            },
            EncodeType::SortBool(_) => {
                if *decode_type != DecodeType::DecodeBool {
                    return false;
                }
            },
            EncodeType::SortF32(_) => {
                if *decode_type != DecodeType::DecodeF32 {
                    return false;
                }
            },
            EncodeType::SortF64(_) => {
                if *decode_type != DecodeType::DecodeF64 {
                    return false;
                }
            },
            EncodeType::SortBytes(_) => {
                if *decode_type != DecodeType::DecodeBytes {
                    return false;
                }
            },
            EncodeType::SortI32(_) => {
                if *decode_type != DecodeType::DecodeI32 {
                    return false;
                }
            },
            EncodeType::SortI64(_) => {
                if *decode_type != DecodeType::DecodeI64 {
                    return false;
                }
            },
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_and_decode() {
        let keys = vec![EncodeType::SortU16(1), EncodeType::SortU32(2)];
        let encoded_data = encode_keys(&keys);
        let the_types = vec![DecodeType::DecodeU16, DecodeType::Reverse, DecodeType::DecodeU32];
        let decoded_data = decode_byte_array(&encoded_data, &the_types);
        assert_eq!(keys, decoded_data);
    }

    #[test]
    fn test_encode_and_decode_string_and_floats() {
        let keys = vec![EncodeType::SortString("hello".to_string()), EncodeType::SortF32(F32struct::new(1.0)), EncodeType::SortF64(F64struct::new(2.0))];
        let encoded_data = encode_keys(&keys);
        let the_types = vec![DecodeType::DecodeString, DecodeType::DecodeF32, DecodeType::DecodeF64];
        let decoded_data = decode_byte_array(&encoded_data, &the_types);
        assert_eq!(keys, decoded_data);
    }

    #[test]
    fn test_encode_and_decode_for_all_types() {
        let keys = vec![EncodeType::SortU16(1), EncodeType::SortU32(2), EncodeType::SortU64(3), EncodeType::SortU128(4), EncodeType::SortI32(5), EncodeType::SortI64(6), EncodeType::SortString("hello".to_string()), EncodeType::SortBytes(vec![1, 2, 3]), EncodeType::SortBool(true), EncodeType::SortF32(F32struct::new(1.0)), EncodeType::SortF64(F64struct::new(2.0))];
        let encoded_data = encode_keys(&keys);
        let the_types = vec![DecodeType::DecodeU16, DecodeType::DecodeU32, DecodeType::DecodeU64, DecodeType::Reverse, DecodeType::DecodeU128, DecodeType::DecodeI32, DecodeType::DecodeI64, DecodeType::DecodeString, DecodeType::DecodeBytes, DecodeType::DecodeBool, DecodeType::DecodeF32, DecodeType::DecodeF64];
        let decoded_data = decode_byte_array(&encoded_data, &the_types);
        assert_eq!(keys, decoded_data);
    }

    #[test]
    fn test_compare() {
        let key1 = vec![EncodeType::SortU16(1), EncodeType::SortU32(2)];
        let key2 = vec![EncodeType::SortU16(1), EncodeType::SortU32(3)];
        let the_types = vec![DecodeType::DecodeU16, DecodeType::DecodeU32];
        let result = compare(&the_types, &key1, &key2);
        assert_eq!(result, Ordering::Less);
    }
    
    #[test]
    fn test_compare_reverse() {
        let key1 = vec![EncodeType::SortU16(1), EncodeType::SortU32(2)];
        let key2 = vec![EncodeType::SortU16(1), EncodeType::SortU32(3)];
        let the_types = vec![DecodeType::DecodeU16, DecodeType::Reverse, DecodeType::DecodeU32];
        let result = compare(&the_types, &key1, &key2);
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn test_compare_string() {
        let key1 = vec![EncodeType::SortString("hello".to_string()), EncodeType::SortF32(F32struct::new(1.0)), EncodeType::SortF64(F64struct::new(2.0))];
        let key2 = vec![EncodeType::SortString("hello".to_string()), EncodeType::SortF32(F32struct::new(1.0)), EncodeType::SortF64(F64struct::new(3.0))];
        let the_types = vec![DecodeType::DecodeString, DecodeType::DecodeF32, DecodeType::DecodeF64];
        let result = compare(&the_types, &key1, &key2);
        assert_eq!(result, Ordering::Less);
    }
    
    #[test]
    fn test_compare_string_reverse() {
        let key1 = vec![EncodeType::SortString("hello".to_string()), EncodeType::SortF32(F32struct::new(1.0)), EncodeType::SortF64(F64struct::new(2.0))];
        let key2 = vec![EncodeType::SortString("hello".to_string()), EncodeType::SortF32(F32struct::new(1.0)), EncodeType::SortF64(F64struct::new(3.0))];
        let the_types = vec![DecodeType::DecodeString, DecodeType::DecodeF32, DecodeType::Reverse, DecodeType::DecodeF64];
        let result = compare(&the_types, &key1, &key2);
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn test_compare_gives_equal() {
        let key1 = vec![EncodeType::SortU16(1), EncodeType::SortU32(2)];
        let key2 = vec![EncodeType::SortU16(1), EncodeType::SortU32(2)];
        let the_types = vec![DecodeType::DecodeU16, DecodeType::DecodeU32];
        let result = compare(&the_types, &key1, &key2);
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn test_compare_gives_equal_reverse() {
        let key1 = vec![EncodeType::SortU16(1), EncodeType::SortU32(2)];
        let key2 = vec![EncodeType::SortU16(1), EncodeType::SortU32(2)];
        let the_types = vec![DecodeType::DecodeU16, DecodeType::Reverse, DecodeType::DecodeU32];
        let result = compare(&the_types, &key1, &key2);
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    // create a function which test the compare_bytes function above
    fn test_compare_bytes_function() {
        let key1 = vec![EncodeType::SortU16(1), EncodeType::SortU32(2)];
        let key2 = vec![EncodeType::SortU16(1), EncodeType::SortU32(3)];
        let the_types = vec![DecodeType::DecodeU16, DecodeType::DecodeU32];
        let encoded_key1 = encode_keys(&key1);
        let encoded_key2 = encode_keys(&key2);
        let result = compare_bytes(&the_types, &encoded_key1, &encoded_key2);
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn test_compare_bytes_function_reverse() {
        let key1 = vec![EncodeType::SortU16(1), EncodeType::SortU32(2)];
        let key2 = vec![EncodeType::SortU16(1), EncodeType::SortU32(3)];
        let the_types = vec![DecodeType::DecodeU16, DecodeType::Reverse, DecodeType::DecodeU32];
        let encoded_key1 = encode_keys(&key1);
        let encoded_key2 = encode_keys(&key2);
        let result = compare_bytes(&the_types, &encoded_key1, &encoded_key2);
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    // write a function which test deserialize_decode_types
    fn test_deserialize_decode_types() {
        let the_types = vec![DecodeType::DecodeU16, DecodeType::DecodeU32, DecodeType::DecodeU64, DecodeType::Reverse, DecodeType::DecodeU128, DecodeType::DecodeI32, DecodeType::DecodeI64, DecodeType::DecodeString, DecodeType::DecodeBytes, DecodeType::DecodeBool, DecodeType::DecodeF32, DecodeType::DecodeF64];
        let serliazed_types = serialize_decode_types(&the_types);
        let decoded_types = deserialize_decode_types(&serliazed_types);
        assert_eq!(the_types, decoded_types);
    }

    // write a test for the validate_types function above
    #[test]
    fn test_validate_types() {
        let the_types = vec![DecodeType::DecodeU16, DecodeType::DecodeU32, DecodeType::DecodeU64, DecodeType::Reverse, DecodeType::DecodeU128, DecodeType::DecodeI32, DecodeType::DecodeI64, DecodeType::DecodeString, DecodeType::DecodeBytes, DecodeType::DecodeBool, DecodeType::DecodeF32, DecodeType::DecodeF64];
        let the_keys = vec![EncodeType::SortU16(1), EncodeType::SortU32(2), EncodeType::SortU64(3), EncodeType::SortU128(4), EncodeType::SortI32(5), EncodeType::SortI64(6), EncodeType::SortString("hello".to_string()), EncodeType::SortBytes(b"hello".to_vec()), EncodeType::SortBool(true), EncodeType::SortF32(F32struct::new(1.0)), EncodeType::SortF64(F64struct::new(2.0))];
        let result = validate_types(&the_types, &the_keys);
        assert!(result);
    }

    #[test]
    fn test_validate_types_fail() {
        let the_types = vec![DecodeType::DecodeU32, DecodeType::DecodeU32, DecodeType::DecodeU64, DecodeType::Reverse, DecodeType::DecodeU128, DecodeType::DecodeI32, DecodeType::DecodeI64, DecodeType::DecodeString, DecodeType::DecodeBytes, DecodeType::DecodeBool, DecodeType::DecodeF32, DecodeType::DecodeF64];
        let the_keys = vec![EncodeType::SortU16(1), EncodeType::SortU32(2), EncodeType::SortU64(3), EncodeType::SortU128(4), EncodeType::SortI32(5), EncodeType::SortI64(6), EncodeType::SortString("hello".to_string()), EncodeType::SortBytes(b"hello".to_vec()), EncodeType::SortBool(true), EncodeType::SortF32(F32struct::new(1.0)), EncodeType::SortF64(F64struct::new(2.0))];
        let result = validate_types(&the_types, &the_keys);
        assert!(!result);
    }
}

