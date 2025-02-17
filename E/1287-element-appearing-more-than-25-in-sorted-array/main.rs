struct Solution {}

impl Solution {
  pub fn find_special_integer(arr: Vec<i32>) -> i32 {
    let step = arr.len() / 4;

    if arr[arr.partition_point(|&x| x < arr[step]) + step] == arr[step] {
      return arr[step];
    }
    if arr[arr.partition_point(|&x| x < arr[step * 2 + 1]) + step] == arr[step * 2 + 1] {
      return arr[step * 2 + 1];
    }
    arr[step * 3 + 2]
  }
}
