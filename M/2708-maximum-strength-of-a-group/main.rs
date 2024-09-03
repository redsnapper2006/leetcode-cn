struct Solution {}

impl Solution {
  pub fn max_strength(nums: Vec<i32>) -> i64 {
    let mut mn: i64 = nums[0] as i64;
    let mut mx = mn;

    (1..nums.len()).for_each(|idx| {
      let v = nums[idx] as i64;
      let t = mn;
      mn = mn.min(v).min(mx * v).min(mn * v);
      mx = mx.max(v).max(mx * v).max(t * v);
    });
    mx
  }

  pub fn max_strength2(nums: Vec<i32>) -> i64 {
    let size = nums.len();
    let mut nums1 = nums.into_iter().filter(|&x| x != 0).collect::<Vec<i32>>();
    if nums1.len() == 0 {
      return 0;
    }
    nums1.sort_unstable();

    let mut offset = match nums1.binary_search(&0) {
      Ok(v) => v,
      Err(e) => e,
    };

    if offset == nums1.len() && nums1.len() == 1 {
      if nums1.len() != size {
        return 0;
      } else {
        return nums1[0] as i64;
      }
    }

    let mut ans: i64 = 1;
    (offset..nums1.len()).for_each(|idx| {
      ans *= nums1[idx] as i64;
    });

    if offset > 0 {
      if offset % 2 == 1 {
        offset -= 1;
      }

      (0..offset).for_each(|idx| {
        ans *= nums1[idx] as i64;
      });
    }
    ans
  }
}

fn main() {
  // println!("{}", Solution::max_strength(vec![3, -1, -5, 2, 5, -9]));
  // println!("{}", Solution::max_strength(vec![-4, -5, -4]));
  println!("{}", Solution::max_strength(vec![-2, -3, 8, 9]));
  println!("{}", Solution::max_strength(vec![0, -1]));
}
