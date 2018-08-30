
use super::format::*;
use std::io::{self, Write};

/// This will take any `Write`-able type as a backing store.
pub struct RawExuBuilder<W> {
    sink: W,
}

impl<W: Write> RawExuBuilder<W> {
    pub fn new(sink: W) -> Self {
        RawExuBuilder {
            sink,
        }
    }

    pub fn build_fs(self) -> FilesystemBuilder<W> {
        FilesystemBuilder(self.sink)
    }
}

pub struct FilesystemBuilder<W>(W);

impl<W: Write> FilesystemBuilder<W> {
    pub fn directory_header_builder(self) -> DirectoryHeaderBuilder<W> {
        DirectoryHeaderBuilder(self.0)
    }
}

pub struct DirectoryHeaderBuilder<W>(W);

impl<W: Write> DirectoryHeaderBuilder<W> {
    pub fn write(&mut self, header: DirectoryHeader) -> io::Result<()> {
        self.0.write_all(
            (&header).into()
        )
    }

    pub fn file_header_builder(self) -> FileHeaderBuilder<W> {
        FileHeaderBuilder(self.0)
    }
}

pub struct FileHeaderBuilder<W>(W);

impl<W: Write> FileHeaderBuilder<W> {
    pub fn write(&mut self, header: FileHeader) -> io::Result<()> {
        self.0.write_all((&header).into())
    }

    pub fn name_section_builder(self) -> NameSectionBuilder<W> {
        NameSectionBuilder(self.0)
    }
}

pub struct NameSectionBuilder<W>(W);

impl<W: Write> NameSectionBuilder<W> {
    pub fn write(&mut self, s: &str) -> io::Result<()> {
        self.0.write_all(s.as_bytes())
    }

    pub fn data_section_builder(self) -> FsDataSectionBuilder<W> {
        FsDataSectionBuilder(self.0)
    }
}

pub struct FsDataSectionBuilder<W>(W);

impl<W: Write> FsDataSectionBuilder<W> {
    pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
        self.0.write_all(data)
    }

    pub fn database_entry_builder(self) -> DatabaseEntryBuilder<W> {
        DatabaseEntryBuilder(self.0)
    }
}

pub struct DatabaseEntryBuilder<W>(W);

impl<W: Write> DatabaseEntryBuilder<W> {
    pub fn write(&mut self, entry: DatabaseEntry) -> io::Result<()> {
        self.0.write_all((&entry).into())
    }

    pub fn db_data_section_builder(self) -> DbDataSectionBuilder<W> {
        DbDataSectionBuilder(self.0)
    }
}

pub struct DbDataSectionBuilder<W>(W);

impl<W: Write> DbDataSectionBuilder<W> {
    pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
        self.0.write_all(data)
    }

    pub fn finish(self) -> W {
        self.0
    }
}