# Notes

## Features

* [x] `dump-header` to parse and view the header.
* [x] `dump-toc` to parse and view the toc.
* [ ] `dump-file` to view all metadata known about a file.
* [ ] `list-files` to list all files in an archive, simlar to `ls` and `ls -lah`.
* [ ] `verify` to check if an archive has any errors.
* [ ] `extract` to extract all (or some) files from an archive.
* [ ] `create` to create an archive from a set of files.
* [ ] `create` to add files to an existing archive.
* [ ] `rebuild` to rebuild an archive (using different checksums, compression and compacting the heap).

## Todo

### [DONE] Change to using xmltree and xml-rs

It's easier.

### Use clap OS strings

Use

    ArgMatches::os_value_of

To get non-valid UTF-8 strings for path names.
