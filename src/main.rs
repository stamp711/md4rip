use itertools::Itertools;
use md4rip::Builder;

fn main() {
    let mut builder = Builder::new();
    match builder.build() {
        Ok((padding, m1, m2)) => {
            println!("Padding: {:02x}", padding.iter().format(""));
            println!("M1: {:02x}", m1.iter().format(""));
            println!("M2: {:02x}", m2.iter().format(""));
        }
        Err(e) => println!("{:?}", e),
    }
}
