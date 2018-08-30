use std::{
    mem,
    slice,
};

#[repr(C, packed)]
pub struct ExuHeader {
    magic: [u8; 4], 	 // ['E', 'X', 'U', '\0']
    version: (u16, u16), // exu format version
    fs_size: u64,		 // size of the included filesystem
    db_size: u64,		 // size of the included filesystem
}

#[repr(C, packed)]
pub struct FileSystemHeader {
    magic: [u8; 6], 		// ['E', 'X', 'U', 'F', 'S', '\0']
    compressed_size: u64, 	// size of compressed fs (in bytes)
    num_dir_headers: u64,	// total number of directory headers
    num_file_headers: u64,	// total number of file headers
    name_section_len: u64,	// size of name section (uncompressed)
    data_section_len: u64,	// size of data section (uncompressed)
}

#[repr(C, packed)]
pub struct DatabaseHeader {
	magic: [u8; 6],			// ['E', 'X', 'U', 'D', 'B', '\0']
    compressed_size: u64,	// size of compressed db (in bytes)
    num_entries: u64,		// total number of database entries.
}

#[repr(C, packed)]
pub struct FileHeader {
    parent_index: u64,	// index of parent dir
    name_offset: u64,	// offset of file name in name section
    name_length: u64,
    data_offset: u64,	// offset of data in data section
    data_length: u64,
    file_data: FileUnion,
    is_symlink: bool,
}

#[repr(C, packed)]
pub union FileUnion {
    file: (u64, u64),	// (offset in data section, data length)
    symlink: u64,		// offset of file_header this links to
}

#[repr(C, packed)]
pub struct DirectoryHeader {
    parent_index: u64,	// index of parent dir (u64::max if none)
	name_offset: u64,	// offset of dir name in name section
    name_length: u64,
    file_header_index: u64, // index of assigned file section
}

#[repr(C, packed)]
pub struct DatabaseEntry {
    key_offset: u64,	// offset of the key in the data section
    key_len: u64,		// length of the key in bytes
    value_offset: u64,	// offset of the value in the data section
    value_len: u64,		// length of the value in bytes
}

macro_rules! sliceable_type {
    ($ty:ident) => {
        impl<'a> From<&'a $ty> for &'a [u8] {
            fn from(t: &'a $ty) -> &'a [u8] {
                let ptr = t as *const _;
                let size = mem::size_of::<$ty>();
                unsafe { slice::from_raw_parts(ptr as *const u8, size) }
            }
        }
    };
}

sliceable_type!(ExuHeader);
sliceable_type!(FileSystemHeader);
sliceable_type!(DatabaseHeader);
sliceable_type!(FileHeader);
sliceable_type!(DirectoryHeader);
sliceable_type!(DatabaseEntry);
