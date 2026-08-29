impl Solution {
  pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
    let n = nums.len();
    let mut arr: Vec<(i32, usize)> = nums.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    arr.sort_unstable();

    let mut ans = vec![0; n];
    let mut left = 0;
    for i in 1..=n {
      if i == n || arr[i].0 - arr[i - 1].0 > limit {
        let mut idx: Vec<usize> = arr[left..i].iter().map(|&(_, j)| j).collect();
        idx.sort_unstable();
        for (k, &j) in idx.iter().enumerate() {
          ans[j] = arr[left + k].0;
        }
        left = i;
      }
    }
    ans
  }
}
