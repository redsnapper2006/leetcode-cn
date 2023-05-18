struct Solution {}

impl Solution {
  pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut buf: Vec<i32> = Vec::new();
    let mut idx1: usize = 0;
    let mut idx2: usize = 0;
    let mut carry: i32 = 0;
    while idx1 < arr1.len() || idx2 < arr2.len() || carry != 0 {
      let mut sum = carry;
      if idx1 < arr1.len() {
        sum += arr1[arr1.len() - 1 - idx1];
      }
      if idx2 < arr2.len() {
        sum += arr2[arr2.len() - 1 - idx2];
      }
      if sum >= 2 {
        buf.push(sum - 2);
        carry = -1;
      } else if sum >= 0 {
        buf.push(sum);
        carry = 0;
      } else {
        buf.push(1);
        carry = 1;
      }
      idx1 += 1;
      idx2 += 1;
    }

    while buf.len() > 1 && buf[buf.len() - 1] == 0 {
      buf.pop();
    }
    buf.reverse();
    buf
  }
}
