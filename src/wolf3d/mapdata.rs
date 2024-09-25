use std::io::BufReader;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct MapData {
    pub plane0: Vec<u8>,
    pub plane1: Vec<u8>,
    pub plane2: Vec<u8>,
    pub width: u16,
    pub height: u16,
    pub name: String,
}

impl MapData {
    pub fn new(
        plane0: Vec<u8>,
        plane1: Vec<u8>,
        plane2: Vec<u8>,
        width: u16,
        height: u16,
        name: String,
    ) -> Self {
        Self {
            plane0,
            plane1,
            plane2,
            width,
            height,
            name,
        }
    }

    pub fn print(self: &Self) {
        println!("=========== Name: {} ===========", self.name);
        let mut plane0buf = BufReader::new(&self.plane0[..]);
        for _ in 0..self.height {
            for _ in 0..self.width {
                let c = plane0buf.read_u16::<LittleEndian>().unwrap();
                if c <= 63 {
                    print!("X");
                } else if c >= 90 && c <= 95 {
                    print!("#");
                } else if c >= 100 && c <= 101 {
                    print!("!");
                } else if c >= 106 && c <= 143 {
                    print!(".");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
