impl Solution {
  pub fn sortable_integers(nums: Vec<i32>) -> i32 {
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    let n = nums.len();

    let mut ans = 0;
    for f in 1..=n {
      if n % f != 0 {
        continue;
      }
      let mut valid = true;
      for i in (0..n).step_by(f) {
        let target = nums[i];
        let mut found = false;
        for diff in 0..f {
          if sorted[i + diff] != target {
            continue;
          }
          if (0..f).all(|offset| sorted[i + (offset + diff) % f] == nums[i + offset]) {
            found = true;
            break;
          }
        }
        if !found {
          valid = false;
          break;
        }
      }
      if valid {
        ans += f as i32;
      }
    }
    ans
  }
}
