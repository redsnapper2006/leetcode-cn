impl Solution {
  pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    derived.iter().sum::<i32>() % 2 == 0
  }

  pub fn does_valid_array_exist2(derived: Vec<i32>) -> bool {
    let mut buf: Vec<i32> = vec![0; derived.len()];

    buf[0] = 0;
    let mut idx: usize = 0;
    while idx < derived.len() {
      if idx < derived.len() - 1 {
        match derived[idx] {
          0 => buf[idx + 1] = buf[idx],
          _ => buf[idx + 1] = 1 - buf[idx],
        }
      } else {
        if derived[idx] == buf[idx] ^ buf[0] {
          return true;
        }
        break;
      }
      idx += 1;
    }

    false
  }
}

struct Solution {}
