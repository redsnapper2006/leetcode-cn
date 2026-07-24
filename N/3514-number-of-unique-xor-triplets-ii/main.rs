impl Solution {
  pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    let n = 1 << 12;

    let mut first: Vec<bool> = vec![false; n];
    for i in 0..nums.len() {
      for j in i..nums.len() {
        first[(nums[i] ^ nums[j]) as usize] = true;
      }
    }

    let mut second: Vec<bool> = vec![false; n];
    for i in 0..n {
      if !first[i] {
        continue;
      }

      for j in 0..nums.len() {
        second[i ^ (nums[j] as usize)] = true;
      }
    }
    second.iter().filter(|&&x| x).count() as i32
  }
}
