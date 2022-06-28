use std::fmt;

use crate::bit::Bit;

pub struct QRCode {
    bits: Vec<Vec<Bit>>,
    pub size: usize,
}

impl QRCode {
    const VERSION_1_SIZE: u8 = 21;

    pub fn new(version: u8) -> Self {
        let size = QRCode::size(version);
        let qr_code = QRCode {
            size,
            bits: QRCode::build_empty_matrix(size),
        };
        qr_code.apply_finder_patterns()

    }

    fn size(version: u8) -> usize {
        (version * 4 + (QRCode::VERSION_1_SIZE - 4)) as usize
    }

    fn build_empty_matrix(size: usize) -> Vec<Vec<Bit>> {
        let row = vec![
            Bit { on: false, reserved: false, }; size
        ];
        vec![row; size]
    }

    fn apply_finder_patterns(mut self) -> QRCode {
        let finder_pattern = QRCode::finder_pattern();
        let finder_pattern = QRCode::build_qr_code_from_pattern(&finder_pattern);
        let finder_pattern_offsets = QRCode::get_finder_pattern_offsets(self.size);

        for (y_offset, x_offset) in finder_pattern_offsets {
            self.add(&finder_pattern, x_offset, y_offset);
        }

        self
    }

    fn get_finder_pattern_offsets(size: usize) -> [(usize, usize); 3] {
        const FINDER_PATTERN_SIZE: usize = 7;
        let offset = size - FINDER_PATTERN_SIZE;
        [ (0, 0), (0, offset), (offset, 0), ]
    }

    fn finder_pattern() -> Vec<Vec<u8>> {
        vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ]
    }

    fn add(&mut self, other: &Self, x_offset: usize, y_offset: usize) {
        let len = other.size;
        for row in 0..len {
            for col in 0..len {
                self.bits[row + y_offset][col + x_offset] += other.bits[row][col];
            }
        }
    }

    fn build_qr_code_from_pattern(pattern: &[Vec<u8>]) -> QRCode {
        QRCode {
            size: pattern.len(),
            bits: QRCode::build_matrix_from_binary_pattern(pattern),
        }
    }

    fn build_matrix_from_binary_pattern(pattern: &[Vec<u8>]) -> Vec<Vec<Bit>> {
        pattern.iter().map(|row|
            row.iter().map(|bit|
                Bit { on: *bit == 1, reserved: true, }
            ).collect()
        ).collect()
    }
}

impl fmt::Display for QRCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();
        for row in &self.bits {
            for bit in row {
                buffer += if bit.on { "  " } else { "██" };
            }
            buffer += "\n";
        }
        write!(f, "{}", buffer)
    }
}

#[cfg(test)]
mod tests {
    mod size {
        use super::super::QRCode;

        #[test]
        fn should_have_size_21_for_version_1() {
            const VERSION_1_SIZE: usize = 21;
            let qr_v1 = QRCode::new(1);

            assert_eq!(qr_v1.size, VERSION_1_SIZE);
        }

        #[test]
        fn should_have_size_25_for_version_2() {
            const VERSION_2_SIZE: usize = 25;
            let qr_v2 = QRCode::new(2);

            assert_eq!(qr_v2.size, VERSION_2_SIZE);
        }


        #[test]
        fn should_have_size_57_for_version_10() {
            const VERSION_10_SIZE: usize = 57;
            let qr_v10 = QRCode::new(10);

            assert_eq!(qr_v10.size, VERSION_10_SIZE);
        }


        #[test]
        fn should_have_size_117_for_version_25() {
            const VERSION_25_SIZE: usize = 117;
            let qr_v25 = QRCode::new(25);

            assert_eq!(qr_v25.size, VERSION_25_SIZE);
        }


        #[test]
        fn should_have_size_177_for_version_40() {
            const VERSION_40_SIZE: usize = 177;
            let qr_v40 = QRCode::new(40);

            assert_eq!(qr_v40.size, VERSION_40_SIZE);
        }
    }

    mod new {
        use super::super::QRCode;

        #[test]
        fn should_have_dimensions_with_length_21_for_version_1() {
            const VERSION_1_DIMENSIONS_LENGTH: usize = 21;

            let qr_v1 = QRCode::new(1);

            assert_eq!(qr_v1.size, VERSION_1_DIMENSIONS_LENGTH);
            assert_eq!(qr_v1.bits[(qr_v1.size - 1)].len(), VERSION_1_DIMENSIONS_LENGTH);
        }

        #[test]
        fn should_have_dimensions_with_length_25_for_version_2() {
            const VERSION_2_DIMENSIONS_LENGTH: usize = 25;

            let qr_v2 = QRCode::new(2);

            assert_eq!(qr_v2.size, VERSION_2_DIMENSIONS_LENGTH);
            assert_eq!(qr_v2.bits[(qr_v2.size - 1)].len(), VERSION_2_DIMENSIONS_LENGTH);
        }

        #[test]
        fn should_have_dimensions_with_length_57_for_version_10() {
            const VERSION_10_DIMENSIONS_LENGTH: usize = 57;

            let qr_v10 = QRCode::new(10);

            assert_eq!(qr_v10.size, VERSION_10_DIMENSIONS_LENGTH);
            assert_eq!(qr_v10.bits[(qr_v10.size - 1)].len(), VERSION_10_DIMENSIONS_LENGTH);
        }

        #[test]
        fn should_have_dimensions_with_length_117_for_version_25() {
            const VERSION_25_DIMENSIONS_LENGTH: usize = 117;

            let qr_v25 = QRCode::new(25);

            assert_eq!(qr_v25.size, VERSION_25_DIMENSIONS_LENGTH);
            assert_eq!(qr_v25.bits[(qr_v25.size - 1)].len(), VERSION_25_DIMENSIONS_LENGTH);
        }

        #[test]
        fn should_have_dimensions_with_length_177_for_version_40() {
            const VERSION_40_DIMENSIONS_LENGTH: usize = 177;

            let qr_v40 = QRCode::new(40);

            assert_eq!(qr_v40.size, VERSION_40_DIMENSIONS_LENGTH);
            assert_eq!(qr_v40.bits[(qr_v40.size - 1)].len(), VERSION_40_DIMENSIONS_LENGTH);

        }
    }

    mod build_matrix_from_binary_pattern {
        use super::super::QRCode;

        #[test]
        fn creates_correct_matrix() {
            let pattern: Vec<Vec<u8>> = vec![
                vec![1, 1],
                vec![1, 0],
            ];

            let generated_bits = QRCode::build_matrix_from_binary_pattern(&pattern);

            assert!(generated_bits[0][0].on);
            assert!(generated_bits[0][1].on);
            assert!(generated_bits[1][0].on);
            assert!(!generated_bits[1][1].on);

            for row in generated_bits {
                for bit in row  {
                    assert!(bit.reserved);
                }
            }
        }

        #[test]
        fn creates_correct_for_one() {
            let pattern: Vec<Vec<u8>> = vec![
                vec![1],
            ];

            let generated_bits = QRCode::build_matrix_from_binary_pattern(&pattern);

            assert!(generated_bits[0][0].on);
            assert_eq!(generated_bits.len(), 1);
            assert_eq!(generated_bits[0].len(), 1);
        }

        #[test]
        fn applies_correct_finder_patterns_for_version_1() {
            let qr_v1 = QRCode::new(1);

            assert!(qr_v1.bits[0][20].on);
            assert!(qr_v1.bits[2][18].on);
            assert!(!qr_v1.bits[2][19].on);
        }

        #[test]
        fn reserves_bits_of_finder_pattern() {
            let qr_v1 = QRCode::new(1);
            assert!(qr_v1.bits[0][20].reserved);
            assert!(qr_v1.bits[2][18].reserved);
            assert!(qr_v1.bits[2][19].reserved);
        }
    }
}
