#![allow(dead_code)]
use std::cmp::Ordering;


pub enum SortType {
    SortU16,
    SortU32,
    SortU64,
    SortU128,
    SortI16,
    SortI32,
    SortI64,
    SortI128,
    SortSortableF32,
    SortSortableF64,
    SortBool,
    SortString,
    SortBytes,
    SortStruct,
    Ascending,
    Descending
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct SortableF32 {
    value: f32
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct SortableF64 {
    value: f64
}

// make trait derive debug
pub trait SortableKey:Clone +  PartialEq + Eq + PartialOrd + Ord + Sized + std::fmt::Debug {
    fn sort_type() -> SortType;
    fn encode(&self) -> Vec<u8>;
    fn decode(str:&[u8]) -> Self;
    fn size(str:&[u8], pos:usize) -> u32; 
}

impl SortableKey for u16 {
    fn sort_type() -> SortType {
        SortType::SortU16
    }
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn decode(bytes: &[u8]) -> Self {
        let mut buf = [0u8; 2];
        buf.copy_from_slice(bytes);
        u16::from_be_bytes(buf)
    }

    fn size(_bytes: &[u8], _pos:usize) -> u32 {
        2
    }
}

impl SortableKey for bool {
    fn sort_type() -> SortType {
        SortType::SortBool
    }
    fn encode(&self) -> Vec<u8> {
        if *self {
            vec![1]
        } else {
            vec![0]
        }
    }
    fn decode(bytes: &[u8]) -> Self {
        bytes[0] == 1 }
    fn size(_bytes: &[u8], _pos:usize) -> u32 {
        1
    }
}


impl SortableKey for SortableF32 {
    fn sort_type() -> SortType {
        SortType::SortSortableF32
    }
    fn encode(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
    fn decode(bytes: &[u8]) -> Self {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(bytes);
        SortableF32 {
            value: f32::from_be_bytes(buf)
        }
    }
    fn size(_bytes: &[u8], _pos:usize) -> u32 {
        4
    }
}

impl SortableKey for SortableF64 {
    fn sort_type() -> SortType {
        SortType::SortSortableF64
    }
    fn encode(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
    fn decode(bytes: &[u8]) -> Self {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(bytes);
        SortableF64 {
            value: f64::from_be_bytes(buf)
        }
    }
    fn size(_bytes: &[u8], _pos:usize) -> u32 {
        8
    }
}

impl SortableKey for u32 {
    fn sort_type() -> SortType {
        SortType::SortU32
    }
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn decode(bytes: &[u8]) -> Self {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(bytes);
        u32::from_be_bytes(buf)
    }
    fn size(_bytes: &[u8], _pos:usize) -> u32 {
        4
    }
}

impl SortableKey for u64 {
    fn sort_type() -> SortType {
        SortType::SortU64
    }
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn decode(bytes: &[u8]) -> Self {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(bytes);
        u64::from_be_bytes(buf)
    }
    fn size(__bytes: &[u8], _pos:usize) -> u32 {
        8
    }
}

impl SortableKey for u128 {
    fn sort_type() -> SortType {
        SortType::SortU128
    }
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn decode(bytes: &[u8]) -> Self {
        let mut buf = [0u8; 16];
        buf.copy_from_slice(bytes);
        u128::from_be_bytes(buf)
    }
    fn size(_bytes: &[u8], _pos:usize) -> u32 {
        16
    }
}

impl SortableKey for i16 {
    fn sort_type() -> SortType {
        SortType::SortI16
    }
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn decode(bytes: &[u8]) -> Self {
        let mut buf = [0u8; 2];
        buf.copy_from_slice(bytes);
        i16::from_be_bytes(buf)
    }
    fn size(_bytes: &[u8], _pos:usize) -> u32 {
        2
    }
}

impl SortableKey for i32 {
    fn sort_type() -> SortType {
        SortType::SortI32
    }
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn decode(bytes: &[u8]) -> Self {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(bytes);
        i32::from_be_bytes(buf)
    }
    fn size(_bytes: &[u8], _pos:usize) -> u32 {
        4
    }
}

impl SortableKey for i64 {
    fn sort_type() -> SortType {
        SortType::SortI64
    }
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn decode(bytes: &[u8]) -> Self {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(bytes);
        i64::from_be_bytes(buf)
    }
    fn size(_bytes: &[u8], _pos:usize) -> u32 {
        8
    }
}

impl SortableKey for i128 {
    fn sort_type() -> SortType {
        SortType::SortI128
    }
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn decode(bytes: &[u8]) -> Self {
        let mut buf = [0u8; 16];
        buf.copy_from_slice(bytes);
        i128::from_be_bytes(buf)
    }
    fn size(_bytes: &[u8], _pos:usize) -> u32 {
        16
    }
}

impl SortableKey for String {
    fn sort_type() -> SortType {
        SortType::SortString
    }
    fn encode(&self) -> Vec<u8> {
         let s = self.as_bytes().to_vec();
         let ls = s.len() as u32;
         [ls.to_be_bytes().to_vec(), s].concat()
    }
    fn decode(bytes: &[u8]) -> Self {
        let len = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        let bytes_rest = &bytes[4..4+len];
        String::from_utf8(bytes_rest.to_vec()).unwrap()
    }

    fn size(str:&[u8], pos:usize) -> u32 {
        let len = u32::from_be_bytes([str[pos], str[pos+1], str[pos+2], str[pos+3]]) as u32;
        len + 4
    }
   
}

impl SortableKey for Vec<u8> {
    fn sort_type() -> SortType {
        SortType::SortBytes
    }
    fn encode(&self) -> Vec<u8> {
        let s = self.to_vec();
        [s.len().to_be_bytes().to_vec(), s].concat()
    }
    fn decode(bytes: &[u8]) -> Self {
        let len = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        let bytes_rest = &bytes[4..4+len];
        bytes_rest.to_vec()
    }
    fn size(bytes: &[u8], pos:usize) -> u32 {
        let len = u32::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]) as u32;
        len + 4
    }
}


fn compare_floats64(a: f64, b: f64) -> Ordering {
    if a.is_nan() || b.is_nan() {
        Ordering::Equal // Consider NaN as equal to any other value for simplicity
    } else if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn compare_floats32(a: f32, b: f32) -> Ordering {
    if a.is_nan() || b.is_nan() {
        Ordering::Equal // Consider NaN as equal to any other value for simplicity
    } else if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

impl PartialEq for SortableF32 {
    fn eq(&self, other: &SortableF32) -> bool {
        compare_floats32(self.value, other.value) == Ordering::Equal
    }
}   

impl PartialEq for SortableF64 {
    fn eq(&self, other: &SortableF64) -> bool {
        compare_floats64(self.value, other.value) == Ordering::Equal
    }
}

impl Eq for SortableF32 {}
impl Eq for SortableF64 {}

impl PartialOrd for SortableF32 {
    fn partial_cmp(&self, other: &SortableF32) -> Option<Ordering> {
        Some(compare_floats32(self.value, other.value))
    }
}

impl PartialOrd for SortableF64 {
    fn partial_cmp(&self, other: &SortableF64) -> Option<Ordering> {
        Some(compare_floats64(self.value, other.value))
    }
}

impl Ord for SortableF32 {
    fn cmp(&self, other: &SortableF32) -> Ordering {
        compare_floats32(self.value, other.value)
    }
}

impl Ord for SortableF64 {
    fn cmp(&self, other: &SortableF64) -> Ordering {
        compare_floats64(self.value, other.value)
    }
}

fn get_sortable_keys<T: SortableKey>(bytes: &[u8]) -> Vec<Box<T>> {
    let mut sortable_keys = Vec::new();
    let mut pos = 0;
    while pos < bytes.len() {
        let size = T::size(bytes, pos) as usize;
        let sortable_key = T::decode(&bytes[pos..pos + size]);
        sortable_keys.push(Box::new(sortable_key));
        pos += size;
    }
    sortable_keys
}

fn get_the_bytes<T:SortableKey>(sortable_keys: &Vec<Box<T>>) -> Vec<u8> {
    let mut bytes = Vec::new();
    for sortable_key in sortable_keys {
        bytes.extend(sortable_key.encode());
    }
    bytes
}

// create a test which tests get_bytes and get_sortable_keys
#[cfg(test)]
mod tests {
    use super::*;

// create a function which tests that get_sortable_keys and get_the_bytes gives same result
fn test_get_bytes<T:SortableKey>(sortable_keys: &Vec<Box<T>>) {
    let bytes = get_the_bytes(sortable_keys);
    let sortable_keys2 = get_sortable_keys(&bytes);
    assert_eq!(sortable_keys, &sortable_keys2);
}

    #[test]
    fn test_get_bytes_i32() {
        let sortable_keys = vec![Box::new(1i32), Box::new(2i32), Box::new(3i32)];
        test_get_bytes(&sortable_keys);
    }

    #[test]
    fn test_get_bytes_i64() {
        let sortable_keys = vec![Box::new(1i64), Box::new(2i64), Box::new(3i64)];
        test_get_bytes(&sortable_keys);
    }

    #[test]
    fn test_get_bytes_i128() {
        let sortable_keys = vec![Box::new(1i128), Box::new(2i128), Box::new(3i128)];
        test_get_bytes(&sortable_keys);
    }

    #[test]
    fn test_get_bytes_string() {
        let sortable_keys = vec![Box::new("a".to_string()), Box::new("b".to_string()), Box::new("c".to_string())];
        // debug print the bytes
        let bytes = get_the_bytes(&sortable_keys);
        println!("{:?}", bytes);
        test_get_bytes(&sortable_keys);
    }


}
