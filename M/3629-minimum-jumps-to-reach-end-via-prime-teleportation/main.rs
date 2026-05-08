use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
  pub fn min_jumps(nums: Vec<i32>) -> i32 {
    fn firstfac(x: i32) -> i32 {
      if x % 2 == 0 {
        return 2;
      };
      for n in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= x) {
        if x % n == 0 {
          return n;
        };
      }
      x
    }

    fn factors_uniq(x: i32) -> Vec<i32> {
      if x <= 1 {
        return vec![];
      };
      let mut lst: Vec<i32> = Vec::new();
      let mut curn = x;
      loop {
        let m = firstfac(curn);
        lst.push(m);
        if curn == m {
          break;
        }
        while curn % m == 0 {
          curn /= m;
        }
        if curn == 1 {
          break;
        }
      }
      lst
    }

    let mut prime_map: HashMap<i32, Vec<usize>> = HashMap::new();
    (0..nums.len()).for_each(|idx| {
      if nums[idx] == 1 {
        return;
      }
      let r = firstfac(nums[idx]);
      if r == nums[idx] {
        prime_map.entry(nums[idx]).or_insert(vec![]).push(idx);
      }
    });
    (0..nums.len()).for_each(|idx| {
      if prime_map.contains_key(&nums[idx]) {
        return;
      }

      let fv = factors_uniq(nums[idx]);
      fv.iter().for_each(|&f| {
        prime_map.entry(f).or_insert(vec![]).push(idx);
      });
    });

    let mut dp: Vec<i32> = vec![i32::MAX; nums.len()];
    dp[0] = 0;
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(0);
    while q.len() > 0 {
      let idx = q.pop_front().unwrap();
      if idx == nums.len() - 1 {
        return dp[idx];
      }
      if prime_map.contains_key(&nums[idx]) {
        for &nxt in prime_map.get(&nums[idx]).unwrap().iter().rev() {
          if nxt == idx || dp[idx] + 1 >= dp[nxt] {
            continue;
          }
          dp[nxt] = dp[idx] + 1;
          q.push_back(nxt)
        }
        prime_map.remove(&nums[idx]);
      }
      if idx > 0 && dp[idx - 1] > dp[idx] + 1 {
        dp[idx - 1] = dp[idx] + 1;
        q.push_back(idx - 1);
      }
      if idx < nums.len() - 1 && dp[idx + 1] > dp[idx] + 1 {
        dp[idx + 1] = dp[idx] + 1;
        q.push_back(idx + 1);
      }
    }

    -1
  }
}
