impl Solution {
  pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
    let mut itvl = intervals;
    itvl.sort_by(|x, y| {
      let x0 = &x[0];
      let y0 = &y[0];
      let x1 = &x[1];
      let y1 = &y[1];
      if x1 != y1 {
        return x1.cmp(y1);
      }
      y0.cmp(x0)
    });

    let mut ans: i32 = 0;
    let mut dp: Vec<(i32, i32)> = vec![];
    itvl.iter().for_each(|v| {
      let s = v[0] - 1;
      let e = v[1];

      let mut sv: i32 = 0;
      let sp = match dp.binary_search_by(|probe| probe.0.cmp(&s)) {
        Ok(v) => v,
        Err(v) => v,
      };
      if dp.len() > 0 {
        if sp < dp.len() && dp[sp].0 == s {
          sv = dp[sp].1;
        } else if sp > 0 {
          sv = dp[sp - 1].1;
        }
      }

      let mut ev: i32 = 0;
      let ep = match dp.binary_search_by(|probe| probe.0.cmp(&e)) {
        Ok(v) => v,
        Err(v) => v,
      };

      if dp.len() > 0 {
        if ep < dp.len() {
          ev = dp[ep].1;
        } else {
          ev = dp[ep - 1].1;
        }
      }

      if ev - sv == 0 {
        dp.push((e - 1, sv + 1));
        dp.push((e, sv + 2));
        ans += 2;
      } else if ev - sv == 1 {
        dp.push((e, ev + 1));
        ans += 1;
      }
    });

    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::intersection_size_two(vec![vec![3, 7], vec![1, 3], vec![8, 9]])
  );
  println!(
    "{}",
    Solution::intersection_size_two(vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]])
  );
  println!(
    "{}",
    Solution::intersection_size_two(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]])
  );
}
