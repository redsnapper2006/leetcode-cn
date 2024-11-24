struct Solution {}

impl Solution {
  pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut cnt: Vec<i32> = vec![0; nums.len()];

    let mut l: Vec<(i32, usize)> = Vec::new();
    nums.iter().enumerate().for_each(|(idx, vv)| {
      vv.iter().for_each(|&v| {
        l.push((v, idx));
      });
    });
    l.sort_by(|x, y| x.0.cmp(&y.0));

    let mut s: usize = 0;
    let mut e: usize = 0;
    let mut ans: Vec<i32> = vec![l[0].0, l[l.len() - 1].0];
    let mut v: i32 = 0;
    let k = nums.len() as i32;
    while e < l.len() {
      cnt[l[e].1] += 1;
      if cnt[l[e].1] == 1 {
        v += 1;
      }
      if v == k {
        while v == k {
          cnt[l[s].1] -= 1;
          if cnt[l[s].1] == 0 {
            v -= 1;
          }
          s += 1;
        }
        if ans[1] - ans[0] > l[e].0 - l[s - 1].0 {
          ans = vec![l[s - 1].0, l[e].0];
        }
      }
      e += 1;
    }
    ans
  }
}
