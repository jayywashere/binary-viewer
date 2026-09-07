# Binary Viewer

> a tiny Rust utility that lets me look at binary files without having to stare at raw bytes like a caveman, BYE I'M LEAVING

## About

ok... this is a little Rust project i made to mess around with reading binary files and displaying their contents in a way that's actually readable .. i hope... .....

it reads the raw bytes from a file and shows them as both hexadecimal values and ASCII characters

very simple. very tiny. very useful for learning how bytes actually work, i think.

## Features

- reads raw bytes from a file
- displays bytes in hexadecimal
- displays printable ASCII characters
- replaces non-printable bytes with `.`
- doesn't do anything fancy because it doesn't need to (minimalistic diva attack: "don't do more than what you need to")

## Requirements

- Rust
- Cargo

## Installation

clone the repository:

```bash
git clone <insert-repository-url-here>
cd binary-viewer
```

then run it with Cargo:

```bash
cargo run
```

## Example

```text
HEX:
48 65 00 FF 41 20 7F

ASCII:
He..A .
```

## Built With

- Rust
- `std::fs`

## License

See [LICENSE](LICENSE).
