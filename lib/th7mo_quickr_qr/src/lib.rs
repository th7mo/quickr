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
mod qrcode {
    mod size {
        use super::super::QRCode;

        #[test]
        fn should_have_size_21_for_version_1() {
            const VERSION_1_SIZE: u8 = 21;
            let qr_v1 = QRCode::new(1);

            assert_eq!(qr_v1.size, VERSION_1_SIZE);
        }

        #[test]
        fn should_have_size_25_for_version_2() {
            const VERSION_2_SIZE: u8 = 25;
            let qr_v2 = QRCode::new(2);

            assert_eq!(qr_v2.size, VERSION_2_SIZE);
        }


        #[test]
        fn should_have_size_57_for_version_10() {
            const VERSION_10_SIZE: u8 = 57;
            let qr_v10 = QRCode::new(10);

            assert_eq!(qr_v10.size, VERSION_10_SIZE);
        }


        #[test]
        fn should_have_size_117_for_version_25() {
            const VERSION_25_SIZE: u8 = 117;
            let qr_v25 = QRCode::new(25);

            assert_eq!(qr_v25.size, VERSION_25_SIZE);
        }


        #[test]
        fn should_have_size_177_for_version_40() {
            const VERSION_40_SIZE: u8 = 177;
            let qr_v40 = QRCode::new(40);

            assert_eq!(qr_v40.size, VERSION_40_SIZE);
        }
    }

    mod new {
        use super::super::QRCode;

        #[test]
        fn should_have_dimentions_with_length_21_for_version_1() {
            const VERSION_1_DIMENTIONS_LENGTH: usize = 21;

            let qr_v1 = QRCode::new(1);

            assert_eq!(qr_v1.bits.len(), VERSION_1_DIMENTIONS_LENGTH);
            assert_eq!(qr_v1.bits[(qr_v1.size - 1) as usize].len(), VERSION_1_DIMENTIONS_LENGTH);
        }

        #[test]
        fn should_have_dimentions_with_length_25_for_version_2() {
            const VERSION_2_DIMENTIONS_LENGTH: usize = 25;

            let qr_v2 = QRCode::new(2);

            assert_eq!(qr_v2.bits.len(), VERSION_2_DIMENTIONS_LENGTH);
            assert_eq!(qr_v2.bits[(qr_v2.size - 1) as usize].len(), VERSION_2_DIMENTIONS_LENGTH);
        }

        #[test]
        fn should_have_dimentions_with_length_57_for_version_10() {
            const VERSION_10_DIMENTIONS_LENGTH: usize = 57;

            let qr_v10 = QRCode::new(10);

            assert_eq!(qr_v10.bits.len(), VERSION_10_DIMENTIONS_LENGTH);
            assert_eq!(qr_v10.bits[(qr_v10.size - 1) as usize].len(), VERSION_10_DIMENTIONS_LENGTH);
        }

        #[test]
        fn should_have_dimentions_with_length_117_for_version_25() {
            const VERSION_25_DIMENTIONS_LENGTH: usize = 117;

            let qr_v25 = QRCode::new(25);

            assert_eq!(qr_v25.bits.len(), VERSION_25_DIMENTIONS_LENGTH);
            assert_eq!(qr_v25.bits[(qr_v25.size - 1) as usize].len(), VERSION_25_DIMENTIONS_LENGTH);
        }

        #[test]
        fn should_have_dimentions_with_length_177_for_version_40() {
            const VERSION_40_DIMENTIONS_LENGTH: usize = 177;

            let qr_v40 = QRCode::new(40);

            assert_eq!(qr_v40.bits.len(), VERSION_40_DIMENTIONS_LENGTH);
            assert_eq!(qr_v40.bits[(qr_v40.size - 1) as usize].len(), VERSION_40_DIMENTIONS_LENGTH);
        }
    }
}
