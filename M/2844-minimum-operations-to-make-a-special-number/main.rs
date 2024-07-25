struct Solution {}

impl Solution {
  pub fn minimum_operations(num: String) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![Vec::new(); 4];
    buf[0].push(-1);
    num.as_bytes().iter().enumerate().for_each(|(idx, &b)| {
      let v = (b - b'0') as i32;
      if v != 0 && v != 5 && v != 2 && v != 7 {
        return;
      }
      let nidx = match v {
        0 => 0,
        2 => 1,
        7 => 3,
        _ => 2,
      };
      buf[nidx].push(idx as i32);
    });

    let mut ans: i32 = num.len() as i32;

    if buf[0].len() > 1 {
      ans = ans.min(
        num.len() as i32 - 1 - buf[0][buf[0].len() - 1] + buf[0][buf[0].len() - 1]
          - buf[0][buf[0].len() - 2]
          - 1,
      );

      if buf[2].len() > 0 {
        let five = buf[2].binary_search(&buf[0][buf[0].len() - 1]).unwrap_err();
        if five > 0 {
          ans = ans.min(
            num.len() as i32 - 1 - buf[0][buf[0].len() - 1] + buf[0][buf[0].len() - 1]
              - buf[2][five - 1]
              - 1,
          );
        }
      }
    }
    if buf[2].len() > 0 {
      if buf[1].len() > 0 {
        let two = buf[1].binary_search(&buf[2][buf[2].len() - 1]).unwrap_err();
        if two > 0 {
          ans = ans.min(
            num.len() as i32 - 1 - buf[2][buf[2].len() - 1] + buf[2][buf[2].len() - 1]
              - buf[1][two - 1]
              - 1,
          );
        }
      }
      if buf[3].len() > 0 {
        let seven = buf[3].binary_search(&buf[2][buf[2].len() - 1]).unwrap_err();
        if seven > 0 {
          ans = ans.min(
            num.len() as i32 - 1 - buf[2][buf[2].len() - 1] + buf[2][buf[2].len() - 1]
              - buf[3][seven - 1]
              - 1,
          );
        }
      }
    }

    ans
  }
}

fn main() {
  // println!("{}", Solution::minimum_operations("2245047".to_string()));
  // println!("{}", Solution::minimum_operations("2908305".to_string()));
  // println!("{}", Solution::minimum_operations("10".to_string()));
  println!("{}", Solution::minimum_operations("75".to_string()));
}
