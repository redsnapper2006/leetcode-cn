impl Solution {
  pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
    let mut arr = arr;
    arr.sort_unstable();

    let mut ans: i64 = 0;
    for e in (2..arr.len()).rev() {
      for i in 0..e - 1 {
        let candi = target - arr[e] - arr[i];
        if candi < arr[i] || candi > arr[e] {
          continue;
        }

        let start = arr.partition_point(|&x| x < candi);
        let end = arr.partition_point(|&x| x < candi + 1);
        ans += (end.min(e) - start.max(i + 1)) as i64;
        ans %= 1000000007;
      }
    }
    ans as _
  }
}
