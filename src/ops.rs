// F acts as a conditional: if X then Y else Z
pub fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

// G acts as a majority function: if at least two on X, Y, Z are on then set bit
pub fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (x & z) | (y & z)
}

// H is the bit-wise XOR "parity" function
pub fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

#[allow(clippy::many_single_char_names)]
pub fn op1(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32) -> u32 {
    a.wrapping_add(f(b, c, d)).wrapping_add(m).rotate_left(s)
}

#[allow(clippy::many_single_char_names)]
pub fn op2(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32) -> u32 {
    a.wrapping_add(g(b, c, d))
        .wrapping_add(m)
        .wrapping_add(0x5A82_7999)
        .rotate_left(s)
}

#[allow(clippy::many_single_char_names)]
pub fn op3(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32) -> u32 {
    a.wrapping_add(h(b, c, d))
        .wrapping_add(m)
        .wrapping_add(0x6ED9_EBA1)
        .rotate_left(s)
}

#[allow(clippy::many_single_char_names)]
pub fn op1_t(v: u32, s: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    v.rotate_right(s).wrapping_sub(a).wrapping_sub(f(b, c, d))
}

#[allow(clippy::many_single_char_names)]
pub fn op2_t(v: u32, s: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    v.rotate_right(s)
        .wrapping_sub(a)
        .wrapping_sub(g(b, c, d))
        .wrapping_sub(0x5A82_7999)
}
