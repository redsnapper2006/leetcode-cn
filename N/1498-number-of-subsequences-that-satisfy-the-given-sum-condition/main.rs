use std::collections::HashMap;
impl Solution {
  pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
    let mut m: HashMap<usize, i64> = HashMap::new();
    let mut cur: usize = 0;
    m.insert(0, 1);
    let mut nums = nums;
    nums.sort_unstable();

    let mut ans: i64 = 0;
    for i in 0..nums.len() {
      let b = nums[i];
      let diff = target - b;
      if diff < b {
        break;
      }
      let off = nums.partition_point(|x| x <= &diff) - 1 - i;

      ans += if m.contains_key(&off) {
        *m.get(&off).unwrap()
      } else {
        let mut base = *m.get(&cur).unwrap();
        while cur < off {
          cur += 1;
          base *= 2;
          base %= 1000000007;
          m.insert(cur, base);
        }
        base
      };
      ans %= 1000000007;
    }

    ans as i32
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::num_subseq(vec![3, 5, 6, 7], 9));
  println!("{}", Solution::num_subseq(vec![3, 3, 6, 8], 10));
  println!("{}", Solution::num_subseq(vec![2, 3, 3, 4, 6, 7], 12));
  println!(
    "{}",
    Solution::num_subseq(
      vec![
        1, 2, 2, 2, 2, 2, 2, 6, 6, 7, 8, 8, 9, 9, 9, 10, 10, 11, 11, 11, 12, 12, 12, 12, 13, 14,
        14, 14, 14, 15, 15, 15, 17, 18, 18, 19, 19, 19, 19, 20, 20, 20, 21, 21, 21, 21, 22, 22, 22,
        23, 24, 24, 26, 27, 27, 28, 28, 28, 29, 30, 30, 30, 30, 30
      ],
      31
    )
  );
}
