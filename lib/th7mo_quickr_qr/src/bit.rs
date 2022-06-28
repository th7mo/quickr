use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Bit {
    pub on: bool,
    pub reserved: bool,
}

impl ops::AddAssign for Bit {
    fn add_assign(&mut self, other: Self) {
        *self = Bit {
            on: if self.reserved { self.on } else { self.on ^ other.on },
            reserved: self.reserved || other.reserved,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Bit;

    #[test]
    fn should_add_on_bit_xor_wise() {
        let mut bit = Bit {
            on: false,
            reserved: false
        };
        bit += Bit {
            on: true,
            reserved: false
        };

        assert!(bit.on);

        bit += Bit {
            on: true,
            reserved: true,
        };

        assert!(!bit.on);
    }

    #[test]
    fn should_not_add_bits_when_bit_reserved() {
        let mut bit = Bit {
            on: false,
            reserved: true,
        };
        bit += Bit {
            on: true,
            reserved: false,
        };

        assert!(!bit.on);
    }

    #[test]
    fn should_not_add_bits_when_bots_bits_reserved() {
        let mut bit = Bit {
            on: false,
            reserved: true,
        };
        bit += Bit {
            on: true,
            reserved: true,
        };

        assert!(!bit.on);
    }
}
