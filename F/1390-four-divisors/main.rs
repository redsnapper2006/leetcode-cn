use std::collections::HashMap;

impl Solution {
  pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
    const C: usize = 100001;
    const C3: i32 = 46;
    let mut is_prime: Vec<i32> = vec![1; C];
    let mut primes: Vec<i32> = vec![];
    for i in 2..C {
      if is_prime[i] == 1 {
        primes.push(i as i32);
      }
      for j in (i + i..C).step_by(i) {
        is_prime[j] = 0;
      }
    }

    let mut factor_map: HashMap<i32, i32> = HashMap::new();
    primes.iter().for_each(|&prime| {
      if prime > C3 {
        return;
      }
      factor_map.insert(
        prime * prime * prime,
        1 + prime + prime * prime + prime * prime * prime,
      );
    });

    for i in 0..primes.len() {
      for j in i + 1..primes.len() {
        if primes[i] > C as i32 / primes[j] {
          break;
        }
        factor_map.insert(
          primes[i] * primes[j],
          1 + primes[i] + primes[j] + primes[i] * primes[j],
        );
      }
    }
    let mut ans: i32 = 0;
    nums.iter().for_each(|num| {
      ans += *factor_map.get(&num).unwrap_or(&0);
    });
    ans
  }

  pub fn sum_four_divisors2(nums: Vec<i32>) -> i32 {
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
