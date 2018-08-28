## The Exu Format

The general format for universal executables is:

```
+--------------+
|  exu header  |
+--------------+
|   fs header  |
+--------------+
|   db header  |
+--------------+

## COMPRESSED #####
                  #
+--------------+  #
|              |  #
|  directory   |  #
|   headers    |  #
|              |  #
+--------------+  #
|              |  #
|     file     |  #
|    headers   |  #
|              |  #
+--------------+  #
|     name     |  #
|    section   |  #
+--------------+  #
|              |  #
|     data     |  #
|    section   |  #
|              |  #
+--------------+  #
                  #
## COMPRESSED #####
                  #
+--------------+  #
|              |  #
|   db_entry   |  #
|    section   |  #
|              |  #
+--------------+  #
|              |  #
|     data     |  #
|    section   |  #
|              |  #
+--------------+  #
                  #
###################
```



All Universal Executables start with a header.

```rust
#[repr(C, packed)]
struct ExuHeader {
    magic: [u8; 4], 	 // ['E', 'X', 'U', '\0']
    version: (u16, u16), // exu format version
    fs_size: u64,		 // size of the included filesystem
    db_size: u64,		 // size of the included filesystem
}
```

This is immediately followed by the file system header (single byte alignment).

```rust
#[repr(C, packed)]
struct FileSystemHeader {
    magic: [u8; 6], 		// ['E', 'X', 'U', 'F', 'S', '\0']
    compressed_size: u64, 	// size of compressed fs (in bytes)
    num_dir_headers: u64,	// total number of directory headers
    num_file_headers: u64,	// total number of file headers
    name_section_len: u64,	// size of name section (uncompressed)
    data_section_len: u64,	// size of data section (uncompressed)
}
```

The filesystem header is followed by the database header.

```rust
#[repr(C, packed)]
struct DatabaseHeader {
	magic: [u8; 6],			// ['E', 'X', 'U', 'D', 'B', '\0']
    compressed_size: u64,	// size of compressed db (in bytes)
    num_entries: u64,		// total number of database entries.
}
```

The rest of the filesystem is compressed with Brotli.

The `FileSystemHeader` is followed by `total_dir_headers` `DirectoryHeader`s.

```rust
#[repr(C, packed)]
struct DirectoryHeader {
    parent_index: u64,	// index of parent dir (u64::max if none)
	name_offset: u64,	// offset of dir name in name section
    name_length: u64,
    file_header_index: u64, // index of assigned file section
}
```

The next section is the file header section, made up of an array of `FileHeader`s.

```rust
#[repr(C, packed)]
struct FileHeader {
    parent_index: u64,	// index of parent dir
    name_offset: u64,	// offset of file name in name section
    name_length: u64,
    data_offset: u64,	// offset of data in data section
    data_length: u64,
    file_data: FileUnion
    is_symlink: bool,
}

#[repr(C, packed)]
union FileUnion {
    file: (u64, u64),	// (offset in data section, data length)
    symlink: u64,		// offset of file_header this links to
}
```

The next section is the name section. This consists of all the names of directories and files concatenated together without padding. They are **not** null-terminated.

The last section of the filesystem is the data section. This consists all the data in all the files concatenated together without padding.



The compressed filesystem is followed by the database (compressed with Brotli) if the `num_entries` field of the `DatabaseHeader` is greater than `0.`

The db_entry section is an array of `DatabaseEntry` structs written in a packed manner.

```rust
#[repr(C, packed)]
struct DatabaseEntry {
    key_offset: u64,	// offset of the key in the data section
    key_len: u64,		// length of the key in bytes
    value_offset: u64,	// offset of the value in the data section
    value_len: u64,		// length of the value in bytes
}
```

