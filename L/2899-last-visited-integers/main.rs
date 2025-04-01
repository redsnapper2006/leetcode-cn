impl Solution {
  pub fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
    let mut buf: Vec<i32> = vec![];
    let mut ans: Vec<i32> = vec![];

    let mut cnt: usize = 0;
    nums.iter().for_each(|&v| {
      if v == -1 {
        cnt += 1;
        if cnt <= buf.len() {
          ans.push(buf[buf.len() - cnt]);
        } else {
          ans.push(-1);
        }
      } else {
        cnt = 0;
        buf.push(v);
      }
    });
    ans
  }
}
