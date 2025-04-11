impl Solution {
  pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
    let mut digits: Vec<i32> = vec![0; 26];
    (0..26).for_each(|v| {
      digits[v] = v as i32 + 1;
    });
    chars
      .as_bytes()
      .into_iter()
      .enumerate()
      .for_each(|(idx, v)| {
        let off = (v - b'a') as usize;
        digits[off] = vals[idx];
      });

    let mut min: i32 = 0;
    let mut sum: i32 = 0;
    let mut ans: i32 = i32::MIN;
    s.as_bytes().into_iter().for_each(|b| {
      let off = (b - b'a') as usize;
      sum += digits[off];
      ans = ans.max(sum - min);
      min = min.min(sum);
    });
    if ans < 0 {
      0
    } else {
      ans
    }
  }
}
