mod viewer;

fn main() {
    std::fs::write(
        "example.bin",
        [0x48, 0x65, 0x00, 0xFF, 0x41, 0x20, 0x7F]
    ).unwrap();
    
    let bv = viewer::BinaryViewer::from_file("example.bin")
        .expect("Failed to read binary file.");

    println!("HEX:");
    println!("{}", bv.get_hex());

    println!("\nASCII");
    println!("{}", bv.get_ascii());
}