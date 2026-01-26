impl Solution {
  pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut arr = arr;
    arr.sort_unstable();
    (1..arr.len()).fold((i32::MAX, vec![]), |(diff, mut ans), idx| {
      if ans.is_empty() || diff == arr[idx] - arr[idx - 1] {
        ans.push(vec![arr[idx - 1], arr[idx]]);
        (diff, ans)
      } else if ans[0][1] - ans[0][0] > arr[idx] - arr[idx - 1] {
        (arr[idx] - arr[idx - 1], vec![arr[idx - 1], arr[idx]])
      }
    })
  }
}
