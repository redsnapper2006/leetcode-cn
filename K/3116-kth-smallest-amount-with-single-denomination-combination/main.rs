impl Solution {
  pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
    let gcd = |x: i64, y: i64| -> i64 {
      let (mut x, mut y) = (x, y);
      while y != 0 {
        let t = y;
        y = x % y;
        x = t;
      }
      x
    };

    let lcm = |x: i64, y: i64| -> i64 { x * y / gcd(x, y) };

    let check = |m: i64, coins: &[i64], k: i64| -> bool {
      let mut cnt: i64 = 0;
      for i in 1..(1 << coins.len()) {
        let mut lcm_buf: i64 = 1;
        let mut is_over: bool = false;
        for j in 0..coins.len() {
          if i & (1 << j) != 0 {
            lcm_buf = lcm(lcm_buf, coins[j]);
            if lcm_buf > m {
              is_over = true;
              break;
            }
          }
        }
        if is_over {
          continue;
        }
        cnt += m / lcm_buf
          * if (i as i64).count_ones() % 2 == 1 {
            1
          } else {
            -1
          };
      }
      cnt >= k
    };

    let k = k as i64;
    let coins = coins.iter().map(|x| *x as i64).collect::<Vec<i64>>();

    let mut start: i64 = k;
    let mut end: i64 = *coins.iter().min().unwrap() * k + 1;
    while start < end {
      let mid = start + (end - start) / 2;
      if check(mid, &coins, k) {
        end = mid;
      } else {
        start = mid + 1;
      }
    }

    start
  }

  pub fn find_kth_smallest2(coins: Vec<i32>, k: i32) -> i64 {
    if coins.iter().any(|&x| x == 1) {
      return k as i64;
    }
    let k = k as usize;
    let coins = coins.iter().map(|x| *x as i64).collect::<Vec<i64>>();
    let mut sum = vec![0i64; coins.len()];
    let mut aggr: Vec<i64> = vec![];
    let mut cnt: usize = 0;
    while cnt < k {
      let mut next: i64 = sum[0] + coins[0];
      let mut idx: Vec<usize> = vec![];
      for i in 0..sum.len() {
        if next > sum[i] + coins[i] {
          next = sum[i] + coins[i];
          idx = vec![i];
        } else if next == sum[i] + coins[i] {
          idx.push(i);
        }
      }
      aggr.push(next);
      if idx.len() == coins.len() {
        break;
      }
      for &i in &idx {
        sum[i] += coins[i];
      }

      cnt += 1;
    }

    let mut last = *aggr.last().unwrap();
    if aggr.len() >= k {
      last
    } else {
      let n = aggr.len();
      last *= (k / n) as i64;
      last + if k % n > 0 { aggr[(k % n) - 1] } else { 0 }
    }
  }
}
