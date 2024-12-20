impl Solution {
  pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
    nums
      .iter()
      .map(|x| {
        let mut x = *x;
        if x == 2 {
          return -1;
        }
        let mut v: Vec<i32> = Vec::new();
        while x > 0 {
          v.push(x % 2);
          x >>= 1;
        }
        let mut idx: usize = 0;
        while idx < v.len() {
          if v[idx] == 0 {
            break;
          }
          idx += 1;
        }
        v[idx - 1] = 0;
        let mut ans: i32 = 0;
        v.iter().rev().for_each(|b| {
          ans = ans * 2 + b;
        });
        ans
      })
      .collect::<Vec<i32>>()
  }
}
