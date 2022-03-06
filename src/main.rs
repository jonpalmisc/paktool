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

fn list_archive(path: &str) {
    let archive = Archive::open(path).unwrap();

    println!("{}:", &path);
    for (i, e) in archive.iter().enumerate() {
        println!("  {:>2} {:#010x} {:>10} {}", i, e.offset, e.size, e.name);
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
