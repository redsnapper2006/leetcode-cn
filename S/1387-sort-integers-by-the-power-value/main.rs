struct Solution {}

impl Solution {
  pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    let mut cache: Vec<i32> = vec![-1; 250505];
    cache[1] = 0;
    fn dfs(n: i32, cache: &mut Vec<i32>) -> i32 {
      if cache[n as usize] != -1 {
        return cache[n as usize];
      }
      let v = if n % 2 == 0 {
        dfs(n / 2, cache)
      } else {
        dfs(n * 3 + 1, cache)
      } + 1;
      cache[n as usize] = v;
      v
    }

    let mut s: Vec<(i32, i32)> = Vec::new();
    (lo..=hi).for_each(|n| {
      dfs(n, &mut cache);
      s.push((cache[n as usize], n));
    });
    s.sort_by(|x, y| {
      if x.0 == y.0 {
        x.1.cmp(&y.1)
      } else {
        x.0.cmp(&y.0)
      }
    });

    s[k as usize - 1].1
  }
}

fn main() {
  println!("{}", Solution::get_kth(1, 1000, 777));
  // println!("{}", Solution::get_kth(7, 11, 4));
}
