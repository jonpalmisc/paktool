#!/usr/bin/env python3
#
# Copyright (c) 2022 Jon Palmisciano. All rights reserved.
#
# Use of this source code is governed by the BSD 3-Clause license; the full
# terms of the license can be found in the LICENSE.txt file.

import sys

USAGE = """Usage: blesstrack.py TRACK

Track files extracted from the game are not editable by default. This is
trivially bypassed by overwriting the file header with a header from a
user-created track. This tool will overwrite a track file in place and replace
its header so that it may be loaded in the track editor.
"""

USER_HEADER = b"\xde\xad\xba\xbe\t\x00\x00\x00,C;\t\x833x\n\x84A\x04\xdf\xef\xbf\xe7\x99\xfd\x00\x00\x00\x00\x00\x00\x00\x00(U\x06\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"


def main():
    if len(sys.argv) < 2 or sys.argv[1] == "-h" or sys.argv[1] == "--help":
        print(USAGE)
        return

    input_path = sys.argv[1]

    with open(input_path, "r+b") as track_file:
        data = track_file.read()

        track_file.seek(0)
        track_file.write(USER_HEADER)
        track_file.write(data[len(USER_HEADER) :])


if __name__ == "__main__":
    main()
