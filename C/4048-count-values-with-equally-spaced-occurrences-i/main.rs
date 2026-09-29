impl Solution {
  pub fn count_special_integers(nums: Vec<i32>) -> i32 {
    let mut pos: Vec<Vec<usize>> = vec![vec![]; 101];
    for i in 0..nums.len() {
      pos[nums[i] as usize].push(i);
    }
    let mut ans = 0i32;
    for i in 0..101 {
      if pos[i].len() == 3 && pos[i][1] - pos[i][0] == pos[i][2] - pos[i][1] {
        ans += 1;
      }
    }
    ans
  }
}
