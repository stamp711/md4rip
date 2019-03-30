#[macro_use]
extern crate lazy_static;

mod builder;
mod collision;
mod common;
mod ops;
mod state;

pub use builder::Builder;

#[cfg(test)]
mod tests {

    #[test]
    fn build_without_prefix() {
        use crate::Builder;
        use itertools::Itertools;
        use md4::{Digest, Md4};

        let mut builder = Builder::new();
        match builder.build() {
            Ok((padding, m1, m2)) => {
                println!("Padding: {:02x}", padding.iter().format(""));
                println!("M1: {:02x}", m1.iter().format(""));
                println!("M2: {:02x}", m2.iter().format(""));

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
        use crate::Builder;
        use itertools::Itertools;
        use md4::{Digest, Md4};
        use rand;

        let mut builder = Builder::new();

        // Create random prefix (1B ~ 4MB)
        let prefix = vec![rand::random(); rand::random::<usize>() % 4095 + 1];
        builder.input(&prefix);

        match builder.build() {
            Ok((padding, m1, m2)) => {
                println!("Padding: {:02x}", padding.iter().format(""));
                println!("M1: {:02x}", m1.iter().format(""));
                println!("M2: {:02x}", m2.iter().format(""));

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
