// Copyright (c) 2022 Jon Palmisciano. All rights reserved.
//
// Use of this source code is governed by the BSD 3-Clause license; the full
// terms of the license can be found in the LICENSE.txt file.

use std::{fs, path::Path};

mod pak;
use pak::*;

fn show_usage_and_quit(cfg: &getopts::Options) -> ! {
    print!("{}", cfg.usage("Usage: paktool [-hle] ARCHIVE"));
    std::process::exit(0);
}

/// File size suffixes, up to gigabytes.
const SIZE_SUFFIXES: &[&str] = &["KB", "MB", "GB"];

/// Converts a size in bytes into a human-friendly display string.
fn display_size(bytes: u32) -> String {
    let mut size = bytes as f32 / 1024.0;
    let mut degree = 0;

    while size > 1024.0 {
        size /= 1024.0;
        degree += 1;
    }

    format!("{:.2} {}", size, SIZE_SUFFIXES[degree])
}

/// Print the entry list of an archive to the terminal.
fn list_archive(path: &str) {
    let archive = Archive::open(path).unwrap();

    for (i, e) in archive.iter().enumerate() {
        let flag = match e.flags {
            0 => 'r',
            1 => 'z',
            _ => '?',
        };

        println!(
            "  {:>4} {:08x} {:>9} ({}) {}",
            i + 1,
            e.offset,
            display_size(e.size),
            flag,
            e.name
        );
    }
}

/// Extract an archive's content in place.
fn extract_archive(path_str: &str, index: usize) {
    let mut archive = Archive::open(path_str).unwrap();

    let mut root_path = Path::new(path_str).to_path_buf();
    root_path.set_extension("");

    for i in 0..archive.entries.len() {
        // If a specific index was requested, skip all other indices.
        if index != 0 && i != index {
            continue;
        }

        let data = archive.entry_data(i).unwrap();
        let entry = &archive[i];

        let entry_path = Path::new(&entry.name);
        let output_file_path = root_path.join(entry_path);

        println!("{}", output_file_path.to_str().unwrap());
        fs::create_dir_all(output_file_path.parent().unwrap()).unwrap();
        fs::write(output_file_path, &data).unwrap();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts_cfg = getopts::Options::new();
    opts_cfg.optflag("h", "help", "Show help and usage info");
    opts_cfg.optflag("l", "list", "List, but do not extract, archive contents");
    opts_cfg.optflag("e", "extract", "Extract the archive in place");
    opts_cfg.optopt(
        "i",
        "index",
        "The entry to extract (all entries by default)",
        "INDEX",
    );

    let opts = opts_cfg.parse(&args[1..]).unwrap();

    // Get the provided input path or show the usage message if missing.
    let input_path = match opts.free.first() {
        Some(p) => p,
        None => show_usage_and_quit(&opts_cfg),
    };

    // Get the requested extraction index if provided, otherwise, default to
    // zero, which is treated as "all entries" by the extraction procedure.
    let index: usize = match opts.opt_get("i") {
        Ok(Some(i)) => i,
        _ => 0,
    };

    if opts.opt_present("l") {
        list_archive(input_path)
    } else if opts.opt_present("e") {
        extract_archive(input_path, index);
    } else {
        show_usage_and_quit(&opts_cfg);
    }
}
