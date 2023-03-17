struct Solution {}

impl Solution {
  pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let size = nums.len();
    let mut n: Vec<i32> = nums;
    n.sort_unstable();
    let mut buf: Vec<i32> = vec![0; size + 1];
    n.iter()
      .enumerate()
      .for_each(|(idx, v)| buf[idx + 1] = buf[idx] + v);
    let mut ret: Vec<i32> = vec![0; queries.len()];
    for (i, v) in queries.iter().enumerate() {
      let mut start: i32 = 0;
      let mut end: i32 = buf.len() as i32 - 1;
      while start <= end {
        let m = start + (end - start) / 2;
        if buf[m as usize] > *v {
          end = m - 1;
        } else {
          start = m + 1;
        }
      }
      ret[i] = start - 1;
    }
    ret
  }
}
