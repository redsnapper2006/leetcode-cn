struct Solution {}

impl Solution {
  pub fn interval_intersection(
    first_list: Vec<Vec<i32>>,
    second_list: Vec<Vec<i32>>,
  ) -> Vec<Vec<i32>> {
    if first_list.len() == 0 || second_list.len() == 0 {
      return Vec::new();
    }

    let mut ret: Vec<Vec<i32>> = Vec::new();
    let mut idx1: usize = 0;
    let mut idx2: usize = 0;
    while idx1 < first_list.len() && idx2 < second_list.len() {
      if first_list[idx1][1] < second_list[idx2][0] {
        idx1 += 1;
        continue;
      }
      if first_list[idx1][0] > second_list[idx2][1] {
        idx2 += 1;
        continue;
      }
      let mut start: i32 = first_list[idx1][0];
      if start < second_list[idx2][0] {
        start = second_list[idx2][0];
      }
      let mut end: i32 = firstList[idx1][1];
      if end > second_list[idx2][1] {
        end = second_list[idx2][1];
        idx2 += 1;
      } else {
        idx1 += 1;
      }
      ret.push(vec![start, end]);
    }
    ret
  }
}
