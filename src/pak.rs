// Copyright (c) 2022 Jon Palmisciano. All rights reserved.
//
// Use of this source code is governed by the BSD 3-Clause license; the full
// terms of the license can be found in the LICENSE.txt file.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::Deref;

use byteorder::*;
use flate2::write::ZlibDecoder;

use crate::util;

/// A PAK archive.
pub struct Archive {
    file: File,
    entries: Vec<Entry>,
}

impl Archive {
    /// Open and parse the archive at the given `path`.
    pub fn open(path: &str) -> Result<Self, std::io::Error> {
        let mut file = std::fs::File::open(path)?;
        let mut entries = Vec::new();

        // Skip over the magic and first offset fields; we don't use them.
        file.seek(SeekFrom::Start(8)).unwrap();

        // Read the entry count, then read all of the entries.
        let entry_count = file.read_u32::<LE>()?;
        for _ in 0..entry_count {
            entries.push(Entry::read_from(&mut file)?);
        }

        // The last entry is the name list; jump to it and read all of the entry
        // names, then update the existing entry list to include name data.
        let name_list_offset = (entries.last().unwrap().offset + 4) as u64;
        file.seek(SeekFrom::Start(name_list_offset))?;
        for i in 0..entry_count - 1 {
            let name_len = file.read_u16::<LE>()?;

            let mut raw_name = vec![0u8; name_len.into()];
            file.read_exact(&mut raw_name)?;

            entries[i as usize].name = String::from_utf8(raw_name).unwrap();
        }

        // The name list shouldn't be treated as a "real entry" and should be
        // removed to prevent accidental extraction, etc.
        entries.remove(entries.len() - 1);

        Ok(Self { file, entries })
    }

    /// Get the (uncompressed_ data for a the entry at the given `index`.
    pub fn entry_data(&mut self, index: usize) -> Result<Vec<u8>, std::io::Error> {
        let entry = &self.entries[index];

        let mut data = Vec::new();
        let mut raw_data = vec![0u8; entry.compressed_size as usize];
        self.file.seek(SeekFrom::Start(entry.offset.into()))?;
        self.file.read_exact(&mut raw_data)?;

        if entry.data_flags == 0 {
            data = raw_data
        } else if entry.data_flags == 1 {
            let mut z = ZlibDecoder::new(data);
            z.write_all(&raw_data).unwrap();
            data = z.finish().unwrap();
        }

        Ok(data)
    }
}

impl Deref for Archive {
    type Target = Vec<Entry>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

/// Entry data flags/types.
struct DataFlags;
impl DataFlags {
    /// Data is uncompressed and raw.
    const RAW: u8 = 0;

    /// Data is compressed with Zlib.
    const ZLIB: u8 = 1;
}

/// An archive entry.
pub struct Entry {
    /// Reserved field, unused by PakTool.
    pub reserved: u32,

    /// Size of the entry's data, when compressed.
    pub compressed_size: u32,

    /// Size of the entry's data, when uncompressed.
    pub raw_size: u32,

    /// Flags regarding the entry's data type and storage.
    pub data_flags: u8,

    /// Offset from archive start to the entry's data.
    pub offset: u32,

    /// Name of the file the entry represents.
    pub name: String,
}

impl Entry {
    /// Read/parse an entry from the given `file`.
    ///
    /// The caller is responsible for seeking the file to the start of the entry
    /// before calling this function.
    pub fn read_from(file: &mut File) -> Result<Self, std::io::Error> {
        let entry = Self {
            reserved: file.read_u32::<LE>()?,
            compressed_size: file.read_u32::<LE>()?,
            raw_size: file.read_u32::<LE>()?,
            data_flags: file.read_u8()?,
            offset: file.read_u32::<LE>()?,
            name: String::new(),
        };

        Ok(entry)
    }

    /// Get the formatted line item for this entry.
    pub fn line_item(&self) -> String {
        let flag = match self.data_flags {
            DataFlags::RAW => 'r',
            DataFlags::ZLIB => 'z',
            _ => '?',
        };

        format!(
            "{:08x} {:>9} ({}) {}",
            self.offset,
            util::display_size(self.raw_size),
            flag,
            self.name
        )
    }
}
