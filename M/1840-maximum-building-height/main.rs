impl Solution {
  pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
    let mut res = restrictions;
    res.push(vec![1, 0]);
    res.sort_unstable();

    fn calc(res1: &Vec<i32>, res2: &Vec<i32>) -> i32 {
      let diff = (res2[0] - res1[0]).abs().min((res2[1] - res1[1]).abs());
      res1[1].min(res2[1]) + diff
    }

    let mut prev: usize = 0;
    for i in 1..res.len() {
      if res[i][1] < res[i - 1][1] {
        continue;
      }

      for j in (prev + 1..i).rev() {
        res[j - 1][1] = calc(&res[j], &res[j - 1]);
      }
      res[i][1] = calc(&res[i - 1], &res[i]);
      prev = i;
    }
    for j in (prev + 1..res.len()).rev() {
      res[j - 1][1] = calc(&res[j], &res[j - 1]);
    }

    prev = res.len() - 1;
    for i in (0..res.len() - 1).rev() {
      if res[i + 1][1] > res[i][1] {
        continue;
      }

      for j in i + 1..prev {
        res[j + 1][1] = calc(&res[j], &res[j + 1]);
      }
      res[i][1] = calc(&res[i + 1], &res[i]);
      prev = i;
    }
    for j in 0..prev {
      res[j + 1][1] = calc(&res[j], &res[j + 1]);
    }

    let mut ans: i32 = 0;
    for i in 1..res.len() {
      ans =
        ans.max(res[i][1].max(res[i - 1][1]) + ((res[i][0] - res[i - 1][0]) - (res[i][1] - res[i - 1][1]).abs()) / 2);
    }
    ans.max(res[res.len() - 1][1] + n - res[res.len() - 1][0])
  }
}
