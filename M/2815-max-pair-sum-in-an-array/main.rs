struct Solution {}

impl Solution {
  pub fn max_sum(nums: Vec<i32>) -> i32 {
    let mut cache: Vec<Vec<i32>> = vec![vec![0; 2]; 10];
    nums.iter().for_each(|&vv| {
      let mut v = vv;
      let mut max: i32 = 0;
      while v > 0 {
        max = max.max(v % 10);
        v /= 10;
      }
      let max = max as usize;
      if cache[max][0] == 0 {
        cache[max][0] = vv;
      } else if cache[max][1] == 0 {
        if cache[max][0] <= vv {
          cache[max][1] = cache[max][0];
          cache[max][0] = vv;
        } else {
          cache[max][1] = vv;
        }
      } else {
        if cache[max][0] <= vv {
          cache[max][1] = cache[max][0];
          cache[max][0] = vv;
        } else if cache[max][1] <= vv {
          cache[max][1] = vv;
        }
      }
    });
    let mut ans: i32 = 0;
    cache.iter().for_each(|ca| {
      if ca[0] > 0 && ca[1] > 0 {
        ans = ans.max(ca[0] + ca[1]);
      }
    });
    if ans == 0 {
      -1
    } else {
      ans
    }
  }
}
