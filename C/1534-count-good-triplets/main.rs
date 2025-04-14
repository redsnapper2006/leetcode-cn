struct Solution {}

impl Solution {
  pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let mut buf: Vec<i32> = vec![0; 1001];
    let mut ans: i32 = 0;
    (0..arr.len()).for_each(|j| {
      (j + 1..arr.len()).for_each(|k| {
        let l = (arr[j] - a).max(arr[k] - c);
        let r = (arr[j] + a).min(arr[k] + c);
        if (arr[j] - arr[k]).abs() > b || l > r {
          return;
        }
        ans += buf[r.min(1000) as usize] - if l <= 0 { 0 } else { buf[l as usize - 1] };
      });

      (arr[j]..1001).for_each(|n| {
        buf[n as usize] += 1;
      });
    });

    ans
  }
}
