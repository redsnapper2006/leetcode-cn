impl Solution {
  pub fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
    if nums.len() == 1 {
      return if k % 2 == 0 { nums[0] } else { -1 };
    }
    if k <= 1 {
      return nums[k as usize];
    }
    if k > nums.len() as i32 {
      return *nums.iter().max().unwrap();
    }
    let candi1 = if nums.len() >= k as usize + 1 {
      nums[k as usize]
    } else {
      -1
    };
    let candi2 = if nums.len() >= k as usize {
      *nums[0..k as usize-1].to_vec().iter().max().unwrap()
    } else {
      -1
    };
    candi1.max(candi2)
  }
}
