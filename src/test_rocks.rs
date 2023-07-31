use rocksdb::{Options, DB};
use std::iter::FromIterator;
use crate::rocks_sortable_keys::*;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compound_comparator() {
    let mut result_vec = Vec::new();
    let the_types = vec![DecodeType::DecodeString, DecodeType::Reverse, DecodeType::DecodeU32];
    let compare_fn =  move |one: &[u8], two: &[u8]| compare_bytes(&the_types, one, two);
    let path = "_path_for_rocksdb_storage";
    {
        let mut db_opts = Options::default();
        db_opts.create_missing_column_families(true);
        db_opts.create_if_missing(true);
        db_opts.set_comparator("cname", Box::new(compare_fn));
        let db = DB::open(&db_opts, path).unwrap();
        // create a byte string that is a compound key, the first part is a string, the second part is a reverse u32 (meaning that the sort order is reversed on the last argument)
        let key1 = vec![EncodeType::SortString("a".to_string()), EncodeType::SortU32(0)];
        let key2 = vec![EncodeType::SortString("a".to_string()), EncodeType::SortU32(1)];
        let key3 = vec![EncodeType::SortString("b".to_string()), EncodeType::SortU32(0)];
        let encoded_key1 = encode_keys(&key1);
        let encoded_key2 = encode_keys(&key2);
        let encoded_key3 = encode_keys(&key3);
 
        db.put(encoded_key1, b"key1").unwrap();
        db.put(encoded_key2, b"key2").unwrap();
        db.put(encoded_key3, b"key3").unwrap();
        let mut iter = db.raw_iterator();
        iter.seek_to_first();
        // collect the values in order
        while iter.valid() {
            let val = iter.value().unwrap();
            let val_str = val.iter().map(|b| *b as char).collect::<Vec<_>>();
            result_vec.push(String::from_iter(val_str));
            iter.next();
        }
    }
    let _ = DB::destroy(&Options::default(), path);
    // assure that key2 comes before key1, and key1 comes before key3
    assert_eq!(result_vec, vec!["key2", "key1", "key3"]);
    }
}
