impl Solution {
  pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    if k == 0 {
      return vec![0; code.len()];
    }

    let mut sums: Vec<i32> = vec![0; code.len()];
    let mut sum: i32 = 0;
    (0..code.len()).for_each(|idx| {
      sum += code[idx];
      sums[idx] = sum;
    });

    let mut ret: Vec<i32> = Vec::new();
    (0..code.len()).for_each(|idx| match k.cmp(&0) {
      std::cmp::Ordering::Greater => {
        let k = k as usize;
        if (idx + k) < code.len() {
          ret.push(sums[idx + k] - sums[idx]);
        } else {
          ret.push(sums[code.len() - 1] - sums[idx] + sums[k + idx - code.len()])
        }
      }
      _ => {
        let kk = (-k) as usize;
        if idx > kk {
          ret.push(sums[idx - 1] - sums[idx - kk - 1]);
        } else {
          if idx > 0 {
            ret.push(sums[idx - 1] + sums[code.len() - 1] - sums[code.len() - 1 - kk + idx]);
          } else {
            ret.push(sums[code.len() - 1] - sums[code.len() - 1 - kk - idx]);
          }
        }
      }
    });
    ret
  }
}
