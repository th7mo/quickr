#[derive(Debug)]
pub struct Bit {
    pub on: bool,
    pub reserved: bool,
}

#[derive(Debug)]
pub struct QRCode {
    version: u8,
    bits: Vec<Vec<Bit>>,
}

impl QRCode {

    pub fn new(version: u8) -> Self {
        QRCode {
            version,
            bits: vec![
                vec![
                    Bit { on: false, reserved: false },
                    Bit { on: false, reserved: false },
                    Bit { on: true, reserved: false },
                ]
            ],
        }
    }

    pub fn size(&self) -> u8 {
        const VERSION_1_SIZE: u8 = 21;
        return (VERSION_1_SIZE - 4) + self.version * 4;
    }
}
