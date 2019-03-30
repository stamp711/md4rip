use crate::collision::CollisionFinder;
use crate::common::*;
use crate::state::MD4State;
use std::io;

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

impl io::Write for Builder {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.input(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod builder_tests {
    use crate::builder::Builder;
    use itertools::Itertools;
    use md4::{Digest, Md4};
    use rand;

    #[test]
    fn build_without_prefix() {
        let mut builder = Builder::new();
        match builder.build() {
            Ok((padding, m1, m2)) => {
                println!("Padding : {:02x}", padding.iter().format(""));
                println!("Message1: {:02x}", m1.iter().format(""));
                println!("Message2: {:02x}", m2.iter().format(""));

                let mut hasher1 = Md4::new();
                let mut hasher2 = Md4::new();

                hasher1.input(&m1);
                hasher2.input(&m2);

                assert_eq!(hasher1.result(), hasher2.result())
            }
            Err(e) => println!("{:?}", e),
        }
    }

    #[test]
    fn build_with_random_prefix() {
        let mut builder = Builder::new();

        // Create random prefix (1B ~ 4MB)
        let prefix = vec![rand::random(); rand::random::<usize>() % 4095 + 1];
        builder.input(&prefix);

        match builder.build() {
            Ok((padding, m1, m2)) => {
                println!("Padding : {:02x}", padding.iter().format(""));
                println!("Message1: {:02x}", m1.iter().format(""));
                println!("Message2: {:02x}", m2.iter().format(""));

                let mut hasher1 = Md4::new();
                let mut hasher2 = Md4::new();

                hasher1.input(&prefix);
                hasher1.input(&padding);
                hasher1.input(&m1);

                hasher2.input(&prefix);
                hasher2.input(&padding);
                hasher2.input(&m2);

                assert_eq!(hasher1.result(), hasher2.result())
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
