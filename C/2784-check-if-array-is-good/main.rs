impl Solution {
  pub fn is_good(nums: Vec<i32>) -> bool {
    let n = nums.len() as i32 - 1;
    let mut aggr: i128 = 0;
    for v in nums {
      if v > n {
        return false;
      }
      match v == n {
        true => {
          aggr += 1 << v;
          if aggr > (1 << (n + 1)) + (1 << n) {
            return false;
          }
        }
        _ => {
          if aggr & (1 << v) > 0 {
            return false;
          }
          aggr += 1 << v;
        }
      }
    }
    aggr + 2 == (1 << (n + 1)) + (1 << n)
  }

  pub fn is_good2(nums: Vec<i32>) -> bool {
    let mut n = nums;

    n.sort();

    let idx: usize = 0;
    while idx < n.len() - 1 {
      if n[idx] != idx + 1 {
        return false;
      }
      idx += 1;
    }
    n[idx] == n.len() - 1
  }
}

struct Solution {}
