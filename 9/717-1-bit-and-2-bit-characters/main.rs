impl Solution {
  pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let mut idx: usize = 0;
    while idx < bits.len() - 1 {
      idx += bits[idx] as usize + 1;
    }
    idx == bits.len() - 1
  }
}
