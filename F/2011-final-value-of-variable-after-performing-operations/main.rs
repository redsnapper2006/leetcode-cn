struct Solution {}

impl Solution {
  pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations.iter().fold(0, |aggr, oper| {
      aggr
        + if oper.chars().nth(1).unwrap() == '+' {
          1
        } else {
          -1
        }
    })
  }

  pub fn final_value_after_operations2(operations: Vec<String>) -> i32 {
    let mut ret: i32 = 0;
    for oper in operations {
      if oper.as_bytes()[1] == '+' as u8 {
        ret += 1;
      } else {
        ret -= 1;
      }
    }
    ret
  }
}
