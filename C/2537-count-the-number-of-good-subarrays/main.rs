use std::collections::HashMap;
impl Solution {
  pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut c: HashMap<i32, i32> = HashMap::new();
    let mut pair: i32 = 0;

    let mut ans: i64 = 0;
    while r < nums.len() {
      while r < nums.len() && pair < k {
        let v = c.entry(nums[r]).and_modify(|x| *x += 1).or_insert(1);
        pair += *v - 1;
        r += 1;
      }
      if pair >= k {
        ans += (nums.len() - r + 1) as i64;
      }

      while l < r && pair >= k {
        let v = c.get_mut(&nums[l]).unwrap();
        *v -= 1;
        pair -= *v;
        if pair >= k {
          ans += (nums.len() - r + 1) as i64;
        }
        l += 1;
      }
    }
    ans
  }
}

struct Solution {}

fn main() {
  // println!("{}", Solution::count_good(vec![1, 1, 1, 1, 1], 10));

  println!("{}", Solution::count_good(vec![3, 1, 4, 3, 2, 2, 4], 2));

  println!(
    "{}",
    Solution::count_good(vec![2, 3, 1, 3, 2, 3, 3, 3, 1, 1, 3, 2, 2, 2], 18)
  );
}
