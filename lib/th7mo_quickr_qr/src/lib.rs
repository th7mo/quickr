#[derive(Debug)]
pub struct Bit {
    pub on: bool,
    pub reserved: bool,
}

#[derive(Debug)]
pub struct QRCode {
    version: u8,
    bits: Vec<Vec<Bit>>,
    pub size: u8,
}

impl QRCode {
    const VERSION_1_SIZE: u8 = 21;

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
            size: QRCode::calc_size(version)
        }
    }

    fn calc_size(version: u8) -> u8 {
        version * 4 + (QRCode::VERSION_1_SIZE - 4)
    }
}

#[cfg(test)]
mod tests {
    use super::QRCode;

    #[test]
    fn should_have_size_21_for_version_1() {
        const VERSION_1_SIZE: u8 = 21;
        let qr_v1 = QRCode::new(1);

        assert_eq!(qr_v1.size, VERSION_1_SIZE);
    }
}
