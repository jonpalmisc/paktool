// Copyright (c) 2022 Jon Palmisciano. All rights reserved.
//
// Use of this source code is governed by the BSD 3-Clause license; the full
// terms of the license can be found in the LICENSE.txt file.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::Deref;

use byteorder::*;
use flate2::write::ZlibDecoder;

/// A PAK archive.
pub struct Archive {
    pub entries: Vec<Entry>,
}

impl Archive {
    /// Open an archive from the disk.
    pub fn open(path: &str) -> Result<Self, std::io::Error> {
        let mut file = std::fs::File::open(path)?;

        let header = Header::read_from(&mut file)?;

        // The entry list starts immediately after the header. Read all of the
        // entries and store a list of them.
        let mut entries = Vec::new();
        for _ in 0..header.entry_count {
            entries.push(Entry::read_from(&mut file)?);
        }

        // The last entry is the name list; jump to it and read all of the entry
        // names, then update the existing entry list to include name data.
        let name_list_offset = (entries.last().unwrap().offset + 4) as u64;
        file.seek(SeekFrom::Start(name_list_offset))?;
        for i in 0..header.entry_count - 1 {
            let name_len = file.read_u16::<LE>()?;
            let mut raw_name = vec![0u8; name_len.into()];
            file.read_exact(&mut raw_name)?;

            entries[i as usize].name = String::from_utf8(raw_name).unwrap();
        }

        // The name list shouldn't be treated as a "real entry" and should be
        // removed to prevent accidental extraction, etc.
        entries.remove(entries.len() - 1);

        Ok(Self { entries })
    }
}

impl Deref for Archive {
    type Target = Vec<Entry>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

#[derive(Default)]
pub struct Header {
    magic: [u8; 4],
    offset: u32,
    entry_count: u32,
}

impl Header {
    pub fn read_from(f: &mut File) -> Result<Self, std::io::Error> {
        let mut header = Self::default();
        f.read_exact(&mut header.magic)?;
        header.offset = f.read_u32::<LE>()?;
        header.entry_count = f.read_u32::<LE>()?;

        Ok(header)
    }
}

pub struct Entry {
    pub reserved: u32,
    pub min_size: u32,
    pub size: u32,
    pub flags: u8,
    pub offset: u32,
    pub name: String,
    pub data: Vec<u8>,
}

impl Entry {
    pub fn read_from(f: &mut File) -> Result<Self, std::io::Error> {
        let mut entry = Self {
            reserved: f.read_u32::<LE>()?,
            min_size: f.read_u32::<LE>()?,
            size: f.read_u32::<LE>()?,
            flags: f.read_u8()?,
            offset: f.read_u32::<LE>()?,
            name: String::new(),
            data: Vec::new(),
        };

        // This method is expected to only advance the file's position by the
        // width of the entry struct. Before the entry's data is read, the
        // current stream position must be saved.
        let saved_pos = f.stream_position()?;

        let mut raw_data = vec![0u8; entry.min_size as usize];
        f.seek(SeekFrom::Start(entry.offset.into()))?;
        f.read_exact(&mut raw_data)?;

        if entry.flags == 0 {
            entry.data = raw_data
        } else if entry.flags == 1 {
            let mut z = ZlibDecoder::new(entry.data);
            z.write_all(&raw_data).unwrap();
            entry.data = z.finish().unwrap();
        }

        // Restore the stream position before returning.
        f.seek(SeekFrom::Start(saved_pos))?;

        Ok(entry)
    }
}
