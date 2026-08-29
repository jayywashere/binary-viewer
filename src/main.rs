mod viewer;

use jstdlib::write_line;
use jstdlib::console::*;

fn main() {
    let filename: String = Console::input("Enter a file path: ");
    
    let bv = viewer::BinaryViewer::from_file(&filename)
        .expect("Failed to read binary file.");

    write_line!("HEX:\n{}", bv.get_hex());
    write_line!("\n\nASCII:\n{}", bv.get_ascii());
}