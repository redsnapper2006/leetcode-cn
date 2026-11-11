impl Solution {
  pub fn make_parity_alternating(nums: Vec<i32>) -> Vec<i32> {
    let mn = *nums.iter().min().unwrap();
    let mx = *nums.iter().max().unwrap();

    let calc = |nums: &[i32], mn: i32, mx: i32, diff: i32| -> (i32, i32) {
      let mut cnt = 0i32;
      let mut mn1 = i32::MAX;
      let mut mx1 = i32::MIN;
      for i in 0..nums.len() {
        let mut val = nums[i];
        if val & 1 != (i as i32 + diff) & 1 {
          cnt += 1;
          if mn == val {
            val += 1;
          } else if mx == val {
            val -= 1;
          }
        }
        mn1 = mn1.min(val);
        mx1 = mx1.max(val);
      }
      (cnt, mx1 - mn1)
    };

    let (odd_cnt, odd_diff) = calc(&nums, mn, mx, 1);
    let (even_cnt, even_diff) = calc(&nums, mn, mx, 0);
    if odd_cnt == even_cnt {
      vec![odd_cnt, odd_diff.min(even_diff)]
    } else if odd_cnt > even_cnt {
      vec![even_cnt, even_diff]
    } else {
      vec![odd_cnt, odd_diff]
    }
  }
}
