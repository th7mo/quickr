use std::ops::AddAssign;

#[derive(Debug)]
pub struct Bit {
    pub on: bool,
    pub reserved: bool,
}

impl Clone for Bit {
    fn clone(&self) -> Self {
        Bit {
            on: self.on,
            reserved: self.reserved,
        }
    }
}

impl Copy for Bit {}

impl AddAssign for Bit {
    fn add_assign(&mut self, other: Self) {
        *self = Bit {
            on: if self.reserved { self.on } else { self.on ^ other.on },
            reserved: self.reserved || other.reserved,
        }
    }
}

#[cfg(test)]
mod tests {
    mod add_assign {
        use super::super::Bit;

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
                reserved: false
            };

            assert!(!bit.on);
        }
    }
}
