// Copyright (c) 2022 Jon Palmisciano. All rights reserved.
//
// Use of this source code is governed by the BSD 3-Clause license; the full
// terms of the license can be found in the LICENSE.txt file.

/// File size suffixes, up to gigabytes.
const SIZE_SUFFIXES: &[&str] = &["KB", "MB", "GB"];

/// Converts a size in bytes into a human-friendly display string.
pub fn display_size(bytes: u32) -> String {
    let mut size = bytes as f32 / 1024.0;
    let mut degree = 0;

    while size > 1024.0 {
        size /= 1024.0;
        degree += 1;
    }

    format!("{:.2} {}", size, SIZE_SUFFIXES[degree])
}
