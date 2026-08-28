use std::fmt::Write;

pub struct BinaryViewer {
    bytes: Vec<u8>
}

impl BinaryViewer {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes
        }
    }

    pub fn from_file(filename: &str) -> std::io::Result<Self> {
        let bytes = std::fs::read(filename)?;
        Ok(Self::new(bytes))
    }

    pub fn get_hex(&self) -> String {
        let mut output = String::new();
        for byte in &self.bytes {
            write!(&mut output, "{byte:02X} ").unwrap();
        }

        output
    }

    pub fn get_ascii(&self) -> String {
        let mut output = String::new();
        for byte in &self.bytes {
            let character = if byte.is_ascii_graphic() || *byte == b' ' {
                *byte as char
            } else {
                '.'
            };
            
            output.push(character);
        }

        output
    }
}