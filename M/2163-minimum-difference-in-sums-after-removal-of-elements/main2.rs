impl Solution {
  pub fn minimum_difference(nums: Vec<i32>) -> i64 {
    let n = nums.len() / 3;

    let mut left: Vec<i64> = vec![];
    let mut left_sum: i64 = 0;
    for i in 0..n {
      let v = nums[i] as i64;
      left.push(v);
      left_sum += v;
    }
    left.sort_unstable();

    let mut right: Vec<(i64, usize)> = vec![];
    for i in n..nums.len() {
      right.push((nums[i] as i64, i));
    }
    right.sort_unstable();
    let mut right_sum: i64 = 0;
    for i in n..2 * n {
      right_sum += right[i].0;
    }

    let mut ans: i64 = left_sum - right_sum;
    let mut right_idx: usize = n;
    for i in n..2 * n {
      let cur = nums[i] as i64;
      if cur < left[n - 1] {
        let off = left.partition_point(|v| *v < cur);
        if off < n {
          let target = left[n - 1];
          left_sum -= target;
          left_sum += cur;
          left.insert(off, cur);
        }
      }

      if cur > right[right_idx].0 || cur == right[right_idx].0 && i >= right[right_idx].1 {
        right_idx -= 1;
        while right_idx > 0 && right[right_idx].1 <= i {
          right_idx -= 1;
          if right_idx == 0 {
            break;
          }
        }

        right_sum += right[right_idx].0 - cur;
      }

      ans = ans.min(left_sum - right_sum);
    }
    ans
  }
}

struct Solution {}

fn main() {
  println!("{} ", Solution::minimum_difference(vec![3, 1, 2]));
  println!("{} ", Solution::minimum_difference(vec![7, 9, 5, 8, 1, 3]));
  println!(
    "{} ",
    Solution::minimum_difference(vec![
      16, 46, 43, 41, 42, 14, 36, 49, 50, 28, 38, 25, 17, 5, 18, 11, 14, 21, 23, 39, 23
    ])
  );
}
