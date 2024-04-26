impl Solution {
  pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |aggr, &v| {
      let mut factor: i32 = 1;
      let mut sum: i32 = 0;
      let mut cnt: i32 = 0;
      while factor * factor <= v {
        if v % factor == 0 {
          cnt += 2;
          if factor == v / factor {
            cnt -= 1;
          }
          if cnt > 4 {
            break;
          }
          sum += factor;
          if factor != v / factor {
            sum += v / factor;
          }
        }
        factor += 1;
      }
      match cnt {
        4 => aggr + sum,
        _ => aggr,
      }
    })
  }
}
