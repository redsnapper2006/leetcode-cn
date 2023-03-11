struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
    let mut m: HashMap<i32, i32> = HashMap::new();

    let mut cnt: i32 = 0;
    m.insert(0, -1);
    let mut start: i32 = 0;
    let mut end: i32 = 0;

    (0..array.len()).for_each(|idx| {
      if array[idx].as_bytes()[0] >= '0' as u8 && array[idx].as_bytes()[0] <= '9' as u8 {
        cnt += 1;
      } else {
        cnt -= 1;
      }

      if m.contains_key(&cnt) {
        if idx as i32 - m.get(&cnt).unwrap() > end - start {
          start = *m.get(&cnt).unwrap();
          end = idx as i32;
        }
      } else {
        m.insert(cnt, idx as i32);
      }
    });

    array[(start + 1) as usize..=end as usize].to_vec()
  }
}
