use itertools::Itertools;
use md4rip::Builder;

fn main() {
    let mut builder = Builder::new();
    let prefix = vec![rand::random(); rand::random::<usize>() % 4095 + 1];
    builder.input(&prefix);
    match builder.build() {
        Ok((padding, m1, m2)) => {
            println!("Padding : {:02x}", padding.iter().format(""));
            println!("Message1: {:02x}", m1.iter().format(""));
            println!("Message2: {:02x}", m2.iter().format(""));
        }
        Err(e) => println!("{:?}", e),
    }
}
