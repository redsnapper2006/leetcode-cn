struct Solution {}

impl Solution {
  pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
    let mut buf: Vec<Vec<i32>> = Vec::new();
    for (i, n) in nums.clone().iter_mut().enumerate() {
      let m = (0..32)
        .map(|_| {
          let m = *n % 2;
          *n /= 2;
          m
        })
        .collect();
      match i {
        0 => buf.push(m),
        _ => buf.push((0..32).map(|idx| m[idx] + buf[i - 1][idx]).collect()),
      };
    }

    let mut ret: Vec<i32> = Vec::new();
    for i in 0..buf.len() {
      let mut base: Vec<i32> = match i {
        0 => vec![0; 32],
        _ => buf[i - 1].clone(),
      };

      let mut diff: Vec<bool> = (0..32)
        .map(|idx| buf[buf.len() - 1][idx] > base[idx])
        .collect();

      let mut s: i32 = i as i32;
      let mut e: i32 = buf.len() as i32 - 1;
      while s <= e {
        let m: i32 = s + (e - s) / 2;
        let mut md: Vec<bool> = (0..32)
          .map(|idx| buf[m as usize][idx] > base[idx])
          .collect();

        if (0..32).map(|idx| diff[idx] == md[idx]).all(|x| x) {
          e = m - 1;
        } else {
          s = m + 1;
        }
      }
      ret.push(s as i32 - i as i32 + 1);
    }

    ret
  }
}

fn main() {
  println!("{:?}", Solution::smallest_subarrays(vec![1, 0, 2, 1, 3]));
}
