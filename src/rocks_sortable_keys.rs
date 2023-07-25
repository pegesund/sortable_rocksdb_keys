#![allow(dead_code)]
// use std::cmp::Ordering;



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncodeType {
    SortU16(u16),
    SortU32(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DecodeType {
    DecodeU16,
    DecodeU32
}

pub trait Encode: std::fmt::Debug + Clone {
    fn encode(&self) -> Vec<u8>;
}

pub trait Decode: std::fmt::Debug + Clone {
    fn decode(data: &[u8], the_type: DecodeType, pos: &mut usize) -> EncodeType;
}

impl Encode for EncodeType {

    fn encode(&self) -> Vec<u8> {
        match self {
            EncodeType::SortU16(value) => value.to_be_bytes().to_vec(),
            EncodeType::SortU32(value) => value.to_be_bytes().to_vec(),
        }
    }
}

impl Decode for DecodeType {

    fn decode(data: &[u8], the_type: DecodeType, pos: &mut usize) -> EncodeType {
        match the_type {
            DecodeType::DecodeU16 => {
                let value = u16::from_be_bytes([data[*pos], data[*pos + 1]]);
                *pos += 2;
                EncodeType::SortU16(value)
            },
            DecodeType::DecodeU32 => {
                let value = u32::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]]);
                *pos += 4;
                EncodeType::SortU32(value)
            }
        }
    }
} 

fn encode_keys<T: Encode>(keys: &[T]) -> Vec<u8> {
    let mut encoded_data = Vec::new();

    for key in keys {
        let encoded_key = key.encode();
        println!("encoded_key: {:?}", encoded_key);
        encoded_data.extend(key.encode());
    }
    encoded_data
}

fn decode_byte_array<T: Decode>(data: &[u8], the_types: &Vec<DecodeType>) -> Vec<EncodeType> {
    let mut pos = 0;
    let mut decoded_data = Vec::new();

    for the_type in the_types {
        decoded_data.push(T::decode(data, the_type.clone(), &mut pos));
    }
    decoded_data
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_and_decode() {
        let keys = vec![EncodeType::SortU16(1), EncodeType::SortU32(2)];
        let encoded_data = encode_keys(&keys);
        println!("encoded_data: {:?}", encoded_data);
        let the_types = vec![DecodeType::DecodeU16, DecodeType::DecodeU32];
        let decoded_data = decode_byte_array::<DecodeType>(&encoded_data, &the_types);
        println!("decoded_data: {:?}", decoded_data);
        assert_eq!(keys, decoded_data);
    }

}

