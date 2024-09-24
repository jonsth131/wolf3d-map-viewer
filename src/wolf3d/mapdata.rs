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
}
