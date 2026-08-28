# Binary Viewer

A small Rust utility that reads a binary file and displays its contents as hexadecimal and ASCII representations.

## Features

* Reads raw bytes from a file
* Displays bytes in hexadecimal
* Displays printable ASCII characters
* Replaces non-printable ASCII bytes with `.`

## Example

```text
HEX:
48 65 00 FF 41 20 7F

ASCII:
He..A .
```

## Usage

Place a binary file named `example.bin` in the project directory, then run:

```bash
cargo run
```

## Built With

* Rust
* `std::fs`