struct Solution {}

impl Solution {
  pub fn count_time(time: String) -> i32 {
    let mut hours: i32 = 1;
    let mut minutes: i32 = 1;
    let question: u8 = '?' as u8;
    let bb: Vec<u8> = time.as_bytes().to_vec();
    if bb[0] == question && bb[1] == question {
      hours = 24;
    } else if bb[0] == question {
      if bb[1] <= '3' as u8 && bb[1] >= '0' as u8 {
        hours = 3
      } else {
        hours = 2
      }
    } else if bb[1] == question {
      if bb[0] == '2' as u8 {
        hours = 4
      } else {
        hours = 10
      }
    }

    if bb[3] == question {
      minutes *= 6
    }
    if bb[4] == question {
      minutes *= 10
    }
    hours * minutes
  }
}
