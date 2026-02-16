impl Solution {
  pub fn reverse_bits(n: i32) -> i32 {
    const M0: u32 = 0x55555555;
    const M1: u32 = 0x33333333;
    const M2: u32 = 0x0F0F0F0F;
    const M3: u32 = 0x00FF00FF;
    const M4: u32 = 0x0000FFFF;

    let mut n = n as u32;
    n = (n >> 1) & M0 | (n & M0) << 1;
    n = (n >> 2) & M1 | (n & M1) << 2;
    n = (n >> 4) & M2 | (n & M2) << 4;
    n = (n >> 8) & M3 | (n & M3) << 8;

    (n >> 16 | (n & M4) << 16) as _
  }
}
