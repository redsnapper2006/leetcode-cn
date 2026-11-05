impl Solution {
  pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
    let mut arr2 = arr2;
    arr2.sort_unstable();

    arr1
      .iter()
      .filter(|&&v| {
        let lo = arr2.partition_point(|&x| x < v - d);
        lo == arr2.len() || arr2[lo] > v + d
      })
      .count() as i32
  }
}
