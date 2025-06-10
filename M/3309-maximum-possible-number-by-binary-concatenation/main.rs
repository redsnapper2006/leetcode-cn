impl Solution {
  pub fn max_good_number(nums: Vec<i32>) -> i32 {
    let mut buf: Vec<(i32, i32)> = vec![];
    nums.iter().for_each(|nn| {
      let mut n = *nn;
      let mut steps: i32 = 0;
      while n > 0 {
        steps += 1;
        n /= 2;
      }
      buf.push((steps, *nn));
    });

    buf.sort_unstable_by(|x, y| ((y.1 << x.0) + x.1).cmp(&((x.1 << y.0) + y.1)));

    let mut ans: i32 = 0;
    buf.iter().for_each(|(steps, n)| {
      ans = ans << steps;
      ans += n;
    });
    ans
  }
}
