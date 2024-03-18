struct Solution {}

struct NumArray {
  s: Vec<i32>,
}

impl NumArray {
  fn new(nums: Vec<i32>) -> Self {
    let mut sum: i32 = 0;
    let mut s: Vec<i32> = Vec::new();
    nums.iter().for_each(|&v| {
      sum += v;
      s.push(sum);
    });
    NumArray { s }
  }

  fn sum_range(&self, left: i32, right: i32) -> i32 {
    if left == 0 {
      self.s[right as usize]
    } else {
      self.s[right as usize] - self.s[left as usize - 1]
    }
  }
}
