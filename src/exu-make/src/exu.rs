use std::io::Write;

use super::raw_exu::*;
use super::format::{
    ExuHeader,
    FileSystemHeader,
    DatabaseHeader,
};

struct InternalFile {
    name: String,
    data: Vec<u8>,
}

struct Directory {
    name: String,
    directories: Vec<Directory>,
    files: Vec<InternalFile>,
}

impl Directory {
    pub fn empty<S>(name: S) -> Self
        where S: Into<String>
    {
        Self {
            directories: Vec::new(),
            files: Vec::new(),
            name: name.into(),
        }
    }
}

pub struct ExuBuilder {
   root: Directory,
}

impl ExuBuilder {
    pub fn new() -> Self {
        ExuBuilder {
            root: Directory::empty("/"),
        }
    }

    pub fn root_directory(&mut self) -> DirectoryBuilder<'_> {
        DirectoryBuilder {
            dir: &mut self.root,
        }
    }

    // compile into an exu file
    pub fn build<W>(self, writer: W) -> W
        where W: Write,
    {

    }
}

pub struct DirectoryBuilder<'dir> {
    dir: &'dir mut Directory,
}

impl<'dir> DirectoryBuilder<'dir> {
    pub fn add_file<S, D>(&mut self, name: S, data: D)
        where
            S: Into<String>,
            D: Into<Vec<u8>>
    {
        self.dir.files.push(InternalFile {
            name: name.into(),
            data: data.into(),
        })
    }

    pub fn add_dir<S>(&mut self, name: S) -> DirectoryBuilder<'_>
        where S: Into<String>
    {
        let dir = Directory {
            name: name.into(),
            directories: Vec::new(),
            files: Vec::new(),  
        };

        self.dir.directories.push(dir);

        DirectoryBuilder {
            dir: self.dir.directories.last_mut().unwrap(),
        }
    }
}