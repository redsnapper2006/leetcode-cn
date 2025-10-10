impl Solution {
  pub fn maximum_energy(mut energy: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut mx: i32 = i32::MIN;
    for i in (0..energy.len()).rev() {
      energy[i] += if i + k < energy.len() {
        energy[i + k]
      } else {
        0
      };
      mx = mx.max(energy[i]);
    }
    mx
  }
}
