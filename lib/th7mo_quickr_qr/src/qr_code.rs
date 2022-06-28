use std::fmt;
use std::ops;

use crate::bit::Bit;

pub struct QRCode {
    pub size: usize,
    has_quiet_zone: bool,
    bits: Vec<Vec<Bit>>,
}

impl QRCode {
    const VERSION_1_SIZE: u8 = 21;
    const TIMING_PATTERN_OFFSET: usize = 10;

    pub fn new(version: u8) -> Self {
        let size = QRCode::size(version);
        let mut qr_code = QRCode {
            size,
            bits: QRCode::build_empty_matrix(size + 8),
            has_quiet_zone: true,
        };
        qr_code.apply_finder_patterns();
        qr_code.apply_timing_patterns();
        qr_code
    }

    fn size(version: u8) -> usize {
        (version * 4 + (QRCode::VERSION_1_SIZE - 4)) as usize
    }

    fn build_empty_matrix(full_size: usize) -> Vec<Vec<Bit>> {
        let row = vec![
            Bit { on: false, reserved: false, }; full_size
        ];
        let mut matrix = vec![row; full_size];

        for (y, row) in matrix.iter_mut().enumerate() {
            for (x, bit) in row.iter_mut().enumerate() {
                if QRCode::is_quiet_zone(y, x, full_size) {
                    bit.reserved = true;
                }
            }
        }

        matrix
    }

    fn is_quiet_zone(row: usize, col: usize, full_size: usize) -> bool {
        row < 4 ||
        row > full_size - 4 ||
        col < 4 ||
        col > full_size - 4
    }

    fn apply_finder_patterns(&mut self) {
        let finder_pattern = QRCode::finder_pattern();
        let finder_pattern = QRCode::build_qr_code_from_pattern(&finder_pattern);
        let finder_pattern_offsets = QRCode::get_finder_pattern_offsets(self.size);

        for (y_offset, x_offset) in finder_pattern_offsets {
            self.add(&finder_pattern, x_offset, y_offset);
        }
    }

    fn get_finder_pattern_offsets(size: usize) -> [(usize, usize); 3] {
        const FINDER_PATTERN_SIZE: usize = 7;
        let offset = size - FINDER_PATTERN_SIZE;
        [ (3, 3), (3, offset + 3), (offset + 3, 3), ]
    }

    fn finder_pattern() -> Vec<Vec<u8>> {
        vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 1, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 1, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 1, 1, 0, 1, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 1, 0],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
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
            has_quiet_zone: false,
        }
    }

    fn build_matrix_from_binary_pattern(pattern: &[Vec<u8>]) -> Vec<Vec<Bit>> {
        pattern.iter().map(|row|
            row.iter().map(|bit|
                Bit { on: *bit == 1, reserved: true, }
            ).collect()
        ).collect()
    }

    fn apply_timing_patterns(&mut self) {
        self.apply_horizontal_timing_pattern(QRCode::TIMING_PATTERN_OFFSET);
        self.apply_vertical_timing_pattern(QRCode::TIMING_PATTERN_OFFSET);
    }

    fn apply_horizontal_timing_pattern(&mut self, row: usize) {
        let row = &mut self.bits[row];

        for (x, bit) in row.iter_mut().enumerate() {
            *bit += Bit {
                on: x % 2 == 0,
                reserved: true,
            };
        }
    }

    fn apply_vertical_timing_pattern(&mut self, col: usize) {
        for (y, row) in self.bits.iter_mut().enumerate() {
            row[col] += Bit {
                on: y % 2 == 0,
                reserved: true,
            };
        }
    }

    fn at(&self, mut row: usize, mut col: usize) -> Bit {
        if self.has_quiet_zone {
            row += 4;
            col += 4;
        }

        self.bits[row][col]
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
    use super::QRCode;

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
        assert_eq!(qr_v2.bits[(qr_v2.size - 1)].len() - 8, VERSION_2_DIMENSIONS_LENGTH);
    }

    #[test]
    fn should_have_dimensions_with_length_57_for_version_10() {
        const VERSION_10_DIMENSIONS_LENGTH: usize = 57;

        let qr_v10 = QRCode::new(10);

        assert_eq!(qr_v10.size, VERSION_10_DIMENSIONS_LENGTH);
        assert_eq!(qr_v10.bits[(qr_v10.size - 1)].len() - 8, VERSION_10_DIMENSIONS_LENGTH);
    }

    #[test]
    fn should_have_dimensions_with_length_117_for_version_25() {
        const VERSION_25_DIMENSIONS_LENGTH: usize = 117;

        let qr_v25 = QRCode::new(25);

        assert_eq!(qr_v25.size, VERSION_25_DIMENSIONS_LENGTH);
        assert_eq!(qr_v25.bits[(qr_v25.size - 1)].len() - 8, VERSION_25_DIMENSIONS_LENGTH);
    }

    #[test]
    fn should_have_dimensions_with_length_177_for_version_40() {
        const VERSION_40_DIMENSIONS_LENGTH: usize = 177;

        let qr_v40 = QRCode::new(40);

        assert_eq!(qr_v40.size, VERSION_40_DIMENSIONS_LENGTH);
        assert_eq!(qr_v40.bits[(qr_v40.size - 1)].len() - 8, VERSION_40_DIMENSIONS_LENGTH);

    }

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

        assert!(qr_v1[0][24].on);
        assert!(qr_v1[2][22].on);
        assert!(!qr_v1[2][23].on);
    }

    #[test]
    fn reserves_bits_of_finder_pattern() {
        let qr_v1 = QRCode::new(1);
        assert!(qr_v1.bits[4][24].reserved);
        assert!(qr_v1.bits[6][22].reserved);
        assert!(qr_v1.bits[6][23].reserved);
    }

    #[test]
    fn adds_timing_pattern() {
        let qr_v1 = QRCode::new(1);

        assert!(qr_v1.bits[10][12].on);
        assert!(qr_v1.bits[10][12].reserved);
        assert!(qr_v1.bits[12][10].on);
        assert!(qr_v1.bits[12][10].reserved);
        assert!(!qr_v1.bits[10][11].on);
        assert!(qr_v1.bits[10][11].reserved);
        assert!(!qr_v1.bits[11][10].on);
        assert!(qr_v1.bits[11][10].reserved);
    }
}
