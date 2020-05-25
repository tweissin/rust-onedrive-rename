# Renames files for OneDrive backup

## Background

When copying files into OneDrive using their web interface, it may complain about files with names that contain certain characters it doesn't like. Unfortunately, the web interface is limited because it doesn't let you copy/paste the offending files. Therefore trying to discover all the files that are causing problems is challenging.

This application solves this issue by swapping the offending characters in the filenames with a hypen '-'.

## Prerequisites

Install Rust as per [this link](https://www.rust-lang.org/tools/install).

## Building 

This is built with Cargo which is package manager like Node's NPM.

To build release version:

`cargo build --release`

This will create a build artifact here: `./target/release/renamer`

## Running

With Cargo:

`cargo run -- -d <dirname>`

With compiled application artifact:

`./target/release/renamer -d <dirname>`

## Debugging

With VSCode, install the Rust and the CodeLLDB extensions.

## Testing

Create a directory named `testdir.templ` with a  structure like this:

```
+ testdir.templ
  - first_:\<>-*"?.rtf
  + subdir
    - second_:\<>-*"?.rtf
    + subsubdir
      - third_:\<>-*"?.rtf
```

Then pass the `-t` option to see if it renames filenames properly in `testdir`.

```
cargo run -- -t -d testdir
```

```
+ testdir.templ
  - first_--------.rtf
  + subdir
    - second_--------.rtf
    + subsubdir
      - third_--------.rtf
```
