struct Solution {}

impl Solution {
  pub fn add_strings(num1: String, num2: String) -> String {
    let mut idx1: i32 = num1.len() as i32 - 1;
    let mut idx2: i32 = num2.len() as i32 - 1;
    let b1 = num1.as_bytes();
    let b2 = num2.as_bytes();
    let mut is_carry = false;
    let mut buf: Vec<u8> = Vec::new();
    while idx1 >= 0 && idx2 >= 0 {
      let n1 = (b1[idx1 as usize] - '0' as u8) as i32;
      let n2 = (b2[idx2 as usize] - '0' as u8) as i32;
      let mut sum = n1 + n2;
      if is_carry {
        sum += 1;
      }
      if sum > 9 {
        sum -= 10;
        is_carry = true;
      } else {
        is_carry = false;
      }
      buf.push((sum as u8 + '0' as u8) as u8);

      idx1 -= 1;
      idx2 -= 1;
    }

    if idx1 >= 0 {
      while idx1 >= 0 {
        let n = (b1[idx1 as usize] - '0' as u8) as i32;
        let mut sum = n;
        if is_carry {
          sum += 1;
        }
        if sum > 9 {
          sum -= 10;
          is_carry = true;
        } else {
          is_carry = false;
        }
        buf.push((sum as u8 + '0' as u8) as u8);

        idx1 -= 1;
      }
    }

    if idx2 >= 0 {
      while idx2 >= 0 {
        let n = (b2[idx2 as usize] - '0' as u8) as i32;
        let mut sum = n;
        if is_carry {
          sum += 1;
        }
        if sum > 9 {
          sum -= 10;
          is_carry = true;
        } else {
          is_carry = false;
        }
        buf.push((sum as u8 + '0' as u8) as u8);
        idx2 -= 1;
      }
    }

    if is_carry {
      buf.push('1' as u8);
    }
    let mut start: usize = 0;
    let mut end: usize = buf.len() - 1;
    while start < end {
      let tmp = buf[start];
      buf[start] = buf[end];
      buf[end] = tmp;
      start += 1;
      end -= 1;
    }
    String::from_utf8(buf).unwrap()
  }
}
