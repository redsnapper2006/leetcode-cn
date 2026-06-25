impl Solution {
  pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
    let mut pos: Vec<i32> = vec![-1; nums.len() + 1];

    let mut cnt: i32 = 0;
    let mut ans: i32 = 0;
    nums.iter().enumerate().for_each(|(idx, &v)| {
      let idx_i32 = idx as i32;
      if v == target {
        cnt += 1;
        pos[cnt as usize] = idx_i32;
      }

      let mut base: i32 = idx_i32;
      for c in 1..=cnt {
        base = base.min(pos[(cnt - c + 1) as usize]);
        let pivot = (idx_i32 - (c * 2 - 1)).max(-1);
        if base > pivot {
          ans += base - pivot;
        }
        base = pivot;
      }
    });

    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::count_majority_subarrays(vec![1, 2, 2, 3], 2)
  );

  println!(
    "{}",
    Solution::count_majority_subarrays(vec![1, 1, 1, 1], 1)
  );
}
