use crate::common::*;
use crate::ops::*;
use byteorder::{ByteOrder, LE};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MD4State {
    pub s: [u32; 4],
}

impl Default for MD4State {
    fn default() -> Self {
        MD4State::new()
    }
}

impl MD4State {
    pub fn new() -> MD4State {
        MD4State {
            s: [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476],
        }
    }

    pub fn apply_block(&mut self, input: &U8Block) {
        *self = self.process_block(input)
    }

    pub fn process_block(&self, input: &U8Block) -> MD4State {
        /* Copy block into data. */
        let mut data = U32Block::default();
        LE::read_u32_into(input, &mut data);

        self.process_u32array(&data)
    }

    pub fn process_u32array(&self, data: &U32Block) -> MD4State {
        let mut a = self.s[0];
        let mut b = self.s[1];
        let mut c = self.s[2];
        let mut d = self.s[3];

        /* Round 1. */
        for &i in &[0, 4, 8, 12] {
            a = op1(a, b, c, d, data[i], 3);
            d = op1(d, a, b, c, data[i + 1], 7);
            c = op1(c, d, a, b, data[i + 2], 11);
            b = op1(b, c, d, a, data[i + 3], 19);
        }

        /* Round 2. */
        for &i in &[0, 1, 2, 3] {
            a = op2(a, b, c, d, data[i], 3);
            d = op2(d, a, b, c, data[i + 4], 5);
            c = op2(c, d, a, b, data[i + 8], 9);
            b = op2(b, c, d, a, data[i + 12], 13);
        }

        /* Round 3. */
        for &i in &[0, 2, 1, 3] {
            a = op3(a, b, c, d, data[i], 3);
            d = op3(d, a, b, c, data[i + 8], 9);
            c = op3(c, d, a, b, data[i + 4], 11);
            b = op3(b, c, d, a, data[i + 12], 15);
        }

        MD4State {
            s: [
                a.wrapping_add(self.s[0]),
                b.wrapping_add(self.s[1]),
                c.wrapping_add(self.s[2]),
                d.wrapping_add(self.s[3]),
            ],
        }
    }
}
