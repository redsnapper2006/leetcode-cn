impl Solution {
  pub fn count_tasks(tasks: Vec<i32>, shifts: Vec<i32>) -> Vec<i32> {
    let mut prefix: Vec<i64> = vec![tasks[0] as i64; tasks.len()];
    for i in 1..tasks.len() {
      prefix[i] = prefix[i - 1] + tasks[i] as i64;
    }

    let mut ans: Vec<i32> = vec![];
    let mut sum: i64 = 0;
    for i in 0..shifts.len() {
      sum += shifts[i] as i64;
      let ll = match prefix.binary_search(&sum) {
        Ok(ov) => ov + 1,
        Err(ev) => ev,
      };
      ans.push((tasks.len() - ll) as i32);
      if ll == tasks.len() {
        sum = 0;
      }
    }
    ans
  }
}
