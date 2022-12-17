struct Solution {}

impl Solution {
  pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
    let mut idx: usize = 0;
    for group in groups {
      if idx + group.len() > nums.len() {
        return false;
      }

      let mut is_found: bool = false;
      while idx + group.len() <= nums.len() {
        if nums[idx] != group[0] {
          idx += 1;
          continue;
        }

        let mut is_match: bool = true;
        for i in 0..group.len() {
          if group[i] != nums[idx + i] {
            is_match = false;
            break;
          }
        }
        if !is_match {
          idx += 1;
        } else {
          is_found = true;
          idx += group.len();
          break;
        }
      }

      if !is_found {
        return false;
      }
    }
    true
  }
}
