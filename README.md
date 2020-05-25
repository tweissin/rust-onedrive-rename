# Renames files for OneDrive backup

## Prerequisites
Install rust as per [this link](https://www.rust-lang.org/tools/install).

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
