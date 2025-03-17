impl Solution {
  pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<i32> = vec![0; 102];

    nums.iter().for_each(|num| {
      buf[num[0] as usize] += 1;
      buf[num[1] as usize + 1] -= 1;
    });

    let mut ans: i32 = 0;
    (1..101).for_each(|idx| {
      buf[idx] += buf[idx - 1];
      ans += if buf[idx] > 0 { 1 } else { 0 };
    });
    ans
  }
}
