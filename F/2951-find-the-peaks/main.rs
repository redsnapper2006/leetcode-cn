impl Solution {
  pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {

    (1..mountain.len() - 1)
      .filter(|&i| mountain[i - 1] < mountain[i] && mountain[i] > mountain[i + 1])
      .map(|i| i as i32)
      .collect::<Vec<i32>>()

  }
}
