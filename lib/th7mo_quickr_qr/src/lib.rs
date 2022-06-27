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
}
