impl Solution {
  pub fn count_kth_roots(l: i32, r: i32, k: i32) -> i32 {
    if k == 1 {
      return r - l + 1;
    }

    fn calc(v: i64, k: u32) -> i32 {
      if v < 0 {
        return 0;
      }
      let mut ans = (v as f64).powf(1.0 / k as f64) as i64;
      if (ans + 1).pow(k) <= v {
        ans += 1;
      }
      ans as i32 + 1
    }

    calc(r as i64, k as u32) - calc(l as i64 - 1, k as u32)
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::count_kth_roots(1, 9, 3));
  println!("{}", Solution::count_kth_roots(8, 30, 2));
  println!("{}", Solution::count_kth_roots(77, 81, 4));
  println!("{}", Solution::count_kth_roots(2, 45, 9));
  println!("{}", Solution::count_kth_roots(19, 22, 1));
  println!("{}", Solution::count_kth_roots(64, 85, 14));
  println!("{}", Solution::count_kth_roots(8, 30, 2));
}
