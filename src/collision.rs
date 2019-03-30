use crate::common::*;
use crate::state::MD4State;
use byteorder::{ByteOrder, LE};
use rand;

pub struct CollisionFinder {
    init: MD4State,
    state: MD4State,
    data: U32Block,
}

enum Kind {
    Equal,
    Zero,
    One,
}
use crate::collision::Kind::Equal;
use crate::ops::{op1, op1_t, op2, op2_t};
use Kind::*;

// Constraints for round 1 & 2
lazy_static! {
    static ref CONSTRAINTS_R1: [Vec<(usize, Kind)>; 16] = [
        /* a1 */ vec![(6, Equal)],
        /* d1 */ vec![(6, Zero), (7, Equal), (10, Equal)],
        /* c1 */ vec![(6, One), (7, One), (10, Zero), (25, Equal)],
        /* b1 */ vec![(6, One), (7, Zero), (10, Zero), (25, Zero)],

        /* a2 */ vec![(7, One), (10, One), (25, Zero), (13, Equal)],
        /* d2 */ vec![(13, Zero), (18, Equal), (19, Equal), (20, Equal),  (21, Equal), (25, One)],
        /* c2 */ vec![(12, Equal), (13, Zero), (14, Equal), (18, Zero), (19, Zero), (20, One), (21, Zero)],
        /* b2 */ vec![(12, One), (13, One), (14, Zero), (16, Equal), (18, Zero), (19, Zero), (20, Zero), (21, Zero)],

        /* a3 */ vec![(12, One), (13, One), (14, One), (16, Zero), (18, Zero), (19, Zero), (20, Zero), (22, Equal), (21, One), (25, Equal)],
        /* d3 */ vec![(12, One), (13, One), (14, One), (16, Zero), (19, Zero), (20, One), (21, One), (22, Zero), (25, One), (29, Equal)],
        /* c3 */ vec![(16, One), (19, Zero), (20, Zero), (21, Zero), (22, Zero), (25, Zero), (29, One), (31, Equal)],
        /* b3 */ vec![(19, Zero), (20, One), (21, One), (22, Equal), (25, One), (29, Zero), (31, Zero)],

        /* a4 */ vec![(22, Zero), (25, Zero), (26, Equal), (28, Equal), (29, One), (31, Zero)],
        /* d4 */ vec![(22, Zero), (25, Zero), (26, One), (28, One), (29, Zero), (31, One)],
        /* c4 */ vec![(18, Equal), (22, One), (25, One), (26, Zero), (28, Zero), (29, Zero)],
        /* b4 */ vec![(18, Zero), (25, Equal), (26, One), (28, One), (29, Zero), (31, Equal)]
    ];

    static ref CONSTRAINTS_A5: [(usize, Kind, usize); 5] = [
        (18, Equal, 2), (25, One, 0), (26, Zero, 0), (28, One, 0), (31, One, 0)
    ];

    static ref CONSTRAINTS_D5: [(usize, Kind, usize); 5] = [
        (18, Equal, 0), (25, Equal, 1), (26, Equal, 1), (28, Equal, 1), (31, Equal, 1)
    ];
}

impl CollisionFinder {
    pub fn from(state: MD4State) -> CollisionFinder {
        CollisionFinder {
            init: state,
            state: Default::default(),
            data: Default::default(),
        }
    }

    fn first_round_single_step(&mut self, step: usize, s: usize, shift: u32) {
        // Calculate chaining variable
        let mut v = op1(
            self.state.s[s % 4],
            self.state.s[(s + 1) % 4],
            self.state.s[(s + 2) % 4],
            self.state.s[(s + 3) % 4],
            self.data[step],
            shift,
        );

        // Adjust chaining variable
        for (digit, kind) in &CONSTRAINTS_R1[step] {
            match kind {
                Equal => v ^= (v ^ self.state.s[(s + 1) % 4]) & (1u32 << digit),
                Zero => v &= !(1u32 << digit),
                One => v |= 1u32 << digit,
            }
        }

        // Adjust data
        self.data[step] = op1_t(
            v,
            shift,
            self.state.s[s % 4],
            self.state.s[(s + 1) % 4],
            self.state.s[(s + 2) % 4],
            self.state.s[(s + 3) % 4],
        );

        // Write v
        self.state.s[s % 4] = v;
    }

    fn second_round_a5(&mut self) {
        // Compute a5
        let mut a5 = op2(
            self.state.s[0],
            self.state.s[1],
            self.state.s[2],
            self.state.s[3],
            self.data[0],
            3,
        );

        // Adjust a5
        for (digit, kind, pos) in CONSTRAINTS_A5.iter() {
            match kind {
                Equal => a5 ^= (a5 ^ self.state.s[*pos]) & (1u32 << digit),
                Zero => a5 &= !(1u32 << digit),
                One => a5 |= 1u32 << digit,
            }
        }

        // Compute new m0 from adjusted a5
        let m0 = op2_t(
            a5,
            3,
            self.state.s[0],
            self.state.s[1],
            self.state.s[2],
            self.state.s[3],
        );

        // Compute original a1..a2
        let [a0, b0, c0, d0] = self.init.s;
        let a1 = op1(a0, b0, c0, d0, self.data[0], 3);
        let d1 = op1(d0, a1, b0, c0, self.data[1], 7);
        let c1 = op1(c0, d1, a1, b0, self.data[2], 11);
        let b1 = op1(b0, c1, d1, a1, self.data[3], 19);
        let a2 = op1(a1, b1, c1, d1, self.data[4], 3);

        // Compute new a1 from m0
        let a1_ = op1(a0, b0, c0, d0, m0, 3);

        // Update m0..m4
        self.data[0] = m0;
        self.data[1] = op1_t(d1, 7, d0, a1_, b0, c0);
        self.data[2] = op1_t(c1, 11, c0, d1, a1_, b0);
        self.data[3] = op1_t(b1, 19, b0, c1, d1, a1_);
        self.data[4] = op1_t(a2, 3, a1_, b1, c1, d1);

        // Write new a5
        self.state.s[0] = a5;
    }

    fn second_round_d5(&mut self) {
        // Compute d5
        let mut d5 = op2(
            self.state.s[3],
            self.state.s[0],
            self.state.s[1],
            self.state.s[2],
            self.data[4],
            5,
        );

        // Adjust d5
        for (digit, kind, pos) in CONSTRAINTS_D5.iter() {
            match kind {
                Equal => d5 ^= (d5 ^ self.state.s[*pos]) & (1u32 << digit),
                Zero => d5 &= !(1u32 << digit),
                One => d5 |= 1u32 << digit,
            }
        }
        //        println!("{}", d5);

        // Compute new m4 from adjusted d5
        let m4 = op2_t(
            d5,
            5,
            self.state.s[3],
            self.state.s[0],
            self.state.s[1],
            self.state.s[2],
        );

        // Compute original a1..a3
        let [a0, b0, c0, d0] = self.init.s;
        let a1 = op1(a0, b0, c0, d0, self.data[0], 3);
        let d1 = op1(d0, a1, b0, c0, self.data[1], 7);
        let c1 = op1(c0, d1, a1, b0, self.data[2], 11);
        let b1 = op1(b0, c1, d1, a1, self.data[3], 19);
        let a2 = op1(a1, b1, c1, d1, self.data[4], 3);
        let d2 = op1(d1, a2, b1, c1, self.data[5], 7);
        let c2 = op1(c1, d2, a2, b1, self.data[6], 11);
        let b2 = op1(b1, c2, d2, a2, self.data[7], 19);
        let a3 = op1(a2, b2, c2, d2, self.data[8], 3);

        // Compute new a2 from m4
        let a2_ = op1(a1, b1, c1, d1, m4, 3);

        // Update m4..m8
        self.data[4] = m4;
        self.data[5] = op1_t(d2, 7, d1, a2_, b1, c1);
        self.data[6] = op1_t(c2, 11, c1, d2, a2_, b1);
        self.data[7] = op1_t(b2, 19, b1, c2, d2, a2_);
        self.data[8] = op1_t(a3, 3, a2_, b2, c2, d2);

        // Write new d5
        self.state.s[3] = d5;
    }

    pub fn find_once(&mut self) -> Option<(U8Block, U8Block)> {
        // Copy init state to state
        self.state = self.init;

        // Generate random message
        for i in &mut self.data {
            *i = rand::random();
        }

        // Convert result into u8 array
        let mut b1 = U8Block::default();
        LE::write_u32_into(&self.data, &mut b1);

        // First round
        let shift = [3, 7, 11, 19];
        let target_s = [0, 3, 2, 1];
        for i in 0..16 {
            self.first_round_single_step(i, target_s[i % 4], shift[i % 4]);
        }

        LE::write_u32_into(&self.data, &mut b1);

        // Second round
        self.second_round_a5();
        self.second_round_d5();

        LE::write_u32_into(&self.data, &mut b1);

        // Create collision message
        self.data[1] = self.data[1].wrapping_add(1 << 31);
        self.data[2] = self.data[2].wrapping_add(1 << 31).wrapping_sub(1 << 28);
        self.data[12] = self.data[12].wrapping_sub(1 << 16);

        let mut b2 = U8Block::default();
        LE::write_u32_into(&self.data, &mut b2);

        if self.init.process_block(&b1) == self.init.process_block(&b2) {
            Some((b1, b2))
        } else {
            None
        }
    }
}
