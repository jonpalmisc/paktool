// Copyright (c) 2022 Jon Palmisciano. All rights reserved.
//
// Use of this source code is governed by the BSD 3-Clause license; the full
// terms of the license can be found in the LICENSE.txt file.

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
        println!(
            "  {:>4}  {:08x}  {:>9}  {}",
            i + 1,
            e.offset,
            display_size(e.size),
            e.name
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts_cfg = getopts::Options::new();
    opts_cfg.optflag("h", "help", "Show help and usage info");
    opts_cfg.optflag("l", "list", "List, but do not extract, archive contents");
    opts_cfg.optflag("e", "extract", "Extract the archive in place");

    let opts = opts_cfg.parse(&args[1..]).unwrap();

    // Get the provided input path or show the usage message if missing.
    let input_path = match opts.free.first() {
        Some(p) => p,
        None => show_usage_and_quit(&opts_cfg),
    };

    if opts.opt_present("l") {
        list_archive(input_path)
    } else if opts.opt_present("e") {
        todo!()
    } else {
        show_usage_and_quit(&opts_cfg);
    }
}
