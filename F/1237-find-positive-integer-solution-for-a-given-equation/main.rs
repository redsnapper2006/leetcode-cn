struct Solution {}

impl Solution {
  pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = Vec::new();
    (1..=1000).for_each(|x| {
      let mut start: i32 = 1;
      let mut end: i32 = 1000;
      while start <= end {
        let m = start + (end - start) / 2;
        let v = customfunction.f(x, m);
        if v < z {
          s = m + 1;
        } else if v > z {
          e = m - 1;
        } else {
          ret.push(vec![x, m]);
          break;
        }
      }
    });

    ret
  }
}
