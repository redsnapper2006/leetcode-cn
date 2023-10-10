impl Solution {
  pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
    let mut n = nums.iter().enumerate().map(|(idx, &v)| {
      let step = match s.as_bytes()[idx] {
        b'R' => 1,
        _ => -1,
      };
      (v  as i64 + step as i64  * d as i64 )
    }).collect::<Vec<i64>>();
    n.sort();
    let mut sum: i64 = 0;
    let mut ans : i64 = 0;
    (0..n.len()).for_each(|s| {
      ans = (ans as i64 + (s as i64 )*n[s] as i64 -sum) % 1000000007 ;
      sum += n[s];
    });
    ans as i32
  }
}
