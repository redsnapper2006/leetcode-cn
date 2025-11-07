impl Solution {
  pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
    let k = k as i64;
    let r = r as usize;
    let ss = stations.len();
    let mut mn: i64 = *stations.iter().min().unwrap() as i64;
    let mut mx: i64 = stations.iter().map(|x| *x as i64).sum::<i64>() + k;

    while mn <= mx {
      let mid = mn + (mx - mn) / 2;

      let mut kk = k;
      let mut sum: i64 = 0;
      let mut aggr: Vec<i64> = vec![0; ss];
      for i in 0..ss + r {
        if i < ss {
          sum += stations[i] as i64;
        }

        if i >= r {
          let diff = sum
            - if i >= 2 * r + 1 {
              aggr[i - 2 * r - 1]
            } else {
              0
            };
          if diff < mid {
            sum += mid - diff;
            kk -= mid - diff;
          }
        }
        if kk < 0 {
          break;
        }
        if i < ss {
          aggr[i] = sum;
        }
      }

      if kk < 0 {
        mx = mid - 1;
      } else {
        mn = mid + 1;
      }
    }

    mx
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::max_power(vec![1, 2, 4, 5, 0], 1, 2));
  println!("{}", Solution::max_power(vec![4,4,4,4], 0, 3));
  println!("{}", Solution::max_power(vec![2, 10, 12, 3], 0, 14));
}
