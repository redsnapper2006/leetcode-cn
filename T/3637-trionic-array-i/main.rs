use std::cmp::Ordering;

impl Solution {
  pub fn is_trionic(nums: Vec<i32>) -> bool {
    let mut fidx: usize = 1;
    while fidx < nums.len() {
      if nums[fidx] == nums[fidx - 1] {
        return false;
      }
      if nums[fidx] < nums[fidx - 1] {
        break;
      }
      fidx += 1;
    }

    let mut tidx: usize = nums.len() - 1;
    while tidx >= 0 {
      if tidx == 0 || nums[tidx] == nums[tidx - 1] {
        return false;
      }
      if nums[tidx] < nums[tidx - 1] {
        break;
      }
      tidx -= 1;
    }

    if fidx == 1 || tidx == nums.len() - 1 {
      return false;
    }
    while fidx < tidx {
      if nums[fidx] <= nums[fidx + 1] {
        return false;
      }
      fidx += 1;
    }
    true
  }

  pub fn is_trionic2(nums: Vec<i32>) -> bool {
    let sort: Vec<std::cmp::Ordering> = vec![Ordering::Greater, Ordering::Less, Ordering::Greater];
    let mut n_idx: usize = 1;
    let mut s_idx: usize = 0;
    while n_idx < nums.len() && s_idx < sort.len() {
      if nums[n_idx].cmp(&nums[n_idx - 1]) == sort[s_idx] {
        n_idx += 1;
      } else if n_idx == 1 || nums[n_idx].cmp(&nums[n_idx - 1]) == Ordering::Equal {
        return false;
      } else {
        s_idx += 1;
      }
    }

    n_idx == nums.len() && s_idx == sort.len() - 1
  }
}
