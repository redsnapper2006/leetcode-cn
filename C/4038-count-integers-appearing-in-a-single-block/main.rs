impl Solution {
  pub fn count_special_integers(nums: Vec<i32>) -> i32 {
    let mut p: [i8; 101] = [-1; 101];
    nums.iter().enumerate().for_each(|(idx, &n)| {
      let n = n as usize;
      if p[n] == -1 || p[n] + 1 == idx as i8 {
        p[n] = idx as i8;
      } else {
        p[n] = -2;
      }
    });
    p.iter().filter(|&x| *x >= 0).count() as i32
  }
}
