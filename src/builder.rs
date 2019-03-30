use crate::collision::CollisionFinder;
use crate::common::*;
use crate::state::MD4State;

#[derive(Default)]
pub struct Builder {
    input_bytes: u64,
    buffer: BlockBuffer<U64>,
    state: MD4State,
    timeout_sec: usize,
}

impl Builder {
    pub fn new() -> Builder {
        Builder::default()
    }

    pub fn input<B: AsRef<[u8]>>(&mut self, input: B) {
        let input = input.as_ref();
        self.input_bytes = self.input_bytes.wrapping_add(input.len() as u64);
        let self_state = &mut self.state;
        self.buffer
            .input(input, |d: &U8Block| self_state.apply_block(d));
    }

    pub fn set_timeout(&mut self, seconds: usize) {
        self.timeout_sec = seconds;
    }

    pub fn build(&mut self) -> Result<(Vec<u8>, U8Block, U8Block), &str> {
        let mut padding = Vec::new();

        // if buffer has remaining, pad with zeros
        let position = self.buffer.position();
        let remaining = self.buffer.remaining();
        if position != 0 {
            padding.extend_from_slice(&vec![0u8; remaining]);
            let self_state = &mut self.state;
            self.buffer
                .input(&padding, |d: &U8Block| self_state.apply_block(d));
        }

        let mut finder = CollisionFinder::from(self.state);
        loop {
            match finder.find_once() {
                Some((m1, m2)) => {
                    return Ok((padding, m1, m2));
                }
                None => continue,
            }
        }
    }
}
