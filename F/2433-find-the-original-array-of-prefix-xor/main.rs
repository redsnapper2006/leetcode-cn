struct Solution {}

impl Solution {
  pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
    pref
      .iter()
      .enumerate()
      .map(|(i, v)| match i {
        0 => *v,
        _ => *v ^ pref[i - 1],
      })
      .collect::<Vec<i32>>()
  }

  pub fn find_array2(pref: Vec<i32>) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();

    let mut sum: i32 = 0;
    for v in pref {
      sum ^= v;
      ret.push(sum);
    }

    ret
  }
}
