impl Solution {
  pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    match mean * (rolls.len() as i32 + n) - rolls.iter().sum::<i32>() {
      remain if remain < n || remain > 6 * n => vec![],
      remain => (0..(remain % n)).fold(vec![remain / n; n as usize], |mut res, i| {
        res[i as usize] += 1;
        res
      }),
    }
  }
}
