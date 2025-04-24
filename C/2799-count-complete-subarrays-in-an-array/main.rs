impl Solution {
  pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    let mut dp_cnt: Vec<i32> = vec![0; 2001];
    let mut unique_cnt: i32 = 0;
    nums.iter().for_each(|&v| {
      let v = v as usize;
      if dp_cnt[v] == 0 {
        unique_cnt += 1;
      }
      dp_cnt[v] += 1;
    });

    dp_cnt = vec![0; 2001];
    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut cnt: i32 = 0;
    let mut ans: i32 = 0;
    while r < nums.len() {
      dp_cnt[nums[r] as usize] += 1;
      if dp_cnt[nums[r] as usize] == 1 {
        cnt += 1;
      }

      while cnt >= unique_cnt && l <= r {
        ans += (nums.len() - r) as i32;
        dp_cnt[nums[l] as usize] -= 1;
        if dp_cnt[nums[l] as usize] == 0 {
          cnt -= 1;
        }
        l += 1;
      }
      r += 1;
    }

    ans
  }
}
