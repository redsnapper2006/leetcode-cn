struct Solution {}

impl Solution {
  pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nn = nums;
    nn.sort();

    let mut res: Vec<Vec<i32>> = Vec::new();

    for idx in (0..nn.len()) {
      if nn[idx] > 0 {
        break;
      }
      if idx > 0 && nn[idx] == nn[idx - 1] {
        continue;
      }

      let mut start = idx + 1;
      let mut end = nn.len() - 1;

      while start < end {
        if start > idx + 1 && start < nn.len() && nn[start] == nn[start - 1] {
          start += 1;
          continue;
        }
        if end < nn.len() - 1 && end >= start && nn[end] == nn[end + 1] {
          end -= 1;
          continue;
        }
        if nn[idx] + nn[start] + nn[end] > 0 {
          end -= 1;
        } else if nn[idx] + nn[start] + nn[end] < 0 {
          start += 1;
        } else {
          res.push(vec![nn[idx], nn[start], nn[end]]);
          start += 1;
          end -= 1;
        }
      }
    }

    res
  }
}
