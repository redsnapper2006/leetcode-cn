
impl Solution {
  pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;

    fn qs(nums: &mut Vec<i32>, k: i32, start: i32, end: i32) -> i32 {
      let prev = start;
      let prev2 = end;
      let mut start = start;
      let mut end = end;

      let base = nums[start as usize];
      while start <= end {
        while start <= end && nums[start as usize] >= base {
          start += 1;
        }
        while start <= end && nums[end as usize] < base {
          end -= 1;
        }
        if start < end {
          let t = nums[start as usize];
          nums[start as usize] = nums[end as usize];
          nums[end as usize] = t;
        }
      }
      let t = nums[start as usize - 1];
      nums[start as usize - 1] = nums[prev as usize];
      nums[prev as usize] = t;
      if start == k {
        return nums[start as usize - 1];
      } else if start > k {
        return qs(nums, k, prev, start - 2);
      } else {
        return qs(nums, k, start, prev2);
      }
    }

    let n = nums.len() as i32;
    qs(&mut nums, k, 0, n - 1)
  }
}
