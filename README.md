# informant
A Rust library and tool for extracting metadata from various media formats.


## Installation


### Exiv2 (XMP) 
```
apt install libgexiv2-dev
```

### Archive

```

INF_DB_FILE=informant.db3 find originals -type f -exec informant {} archive --directory informant \;


INF_DB_FILE=informant.db3 find originals -type f -exec informant {} archive --create-database true --directory informant \;

RUST_LOG=info INF_DB_FILE=informant.db3 find processing/png -type f -exec informant {} archive --directory informant \;

```