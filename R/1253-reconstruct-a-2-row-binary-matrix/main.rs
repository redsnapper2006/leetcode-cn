struct Solution {}

impl Solution {
  pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let mut two_cnt: i32 = 0;
    let mut one_cnt: i32 = 0;
    colsum.clone().into_iter().for_each(|v| {
      if v == 2 {
        two_cnt += 1;
      }
      if v == 1 {
        one_cnt += 1;
      }
    });
    if lower < two_cnt || upper < two_cnt || one_cnt != upper + lower - two_cnt - two_cnt {
      return Vec::new();
    }

    let mut up: Vec<i32> = Vec::new();
    let mut lo: Vec<i32> = Vec::new();
    let mut upper_cnt: i32 = upper - two_cnt;
    colsum.iter().for_each(|&v| {
      if v == 2 {
        up.push(1);
        lo.push(1);
      } else if v == 0 {
        up.push(0);
        lo.push(0);
      } else {
        if upper_cnt > 0 {
          up.push(1);
          lo.push(0);
          upper_cnt -= 1;
        } else {
          up.push(0);
          lo.push(1);
        }
      }
    });

    vec![up, lo]
  }
}

fn main() {
  println!("{:?}", Solution::reconstruct_matrix(2, 1, vec![1, 2, 1]));
}
