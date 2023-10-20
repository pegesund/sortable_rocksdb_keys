# sortable_rocksdb_keys
Adds sortable functionality for the rocksdb key side

This library adds functionality for having sortable, compound keys in rocksdb.
It supports most basic types like integers, doubles and strings - as well as support for sorting ascending or descending.

For example to define a sort function which is based on first a string and then a descending u32 we define a list and set the compare function like this.

```
    let the_types = vec![DecodeType::DecodeString, DecodeType::Reverse, DecodeType::DecodeU32];
    let compare_fn =  move |one: &[u8], two: &[u8]| compare_bytes(&the_types, one, two);
    let mut db_opts = Options::default();
    db_opts.create_missing_column_families(true);
    db_opts.create_if_missing(true);
    db_opts.set_comparator("cname", Box::new(compare_fn));
```

This would be like creating an index in sql like 

```
create index INDEX_NAME on mytable(the_string, the_u32 DESC)
```

The sort-function will add a some extra time on insertion/sort, but it is made for high speed and is also completely pure.

A complete example can be seen in test example in the repo.
