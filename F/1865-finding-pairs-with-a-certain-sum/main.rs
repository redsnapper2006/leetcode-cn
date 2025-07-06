use std::collections::HashMap;
struct FindSumPairs {
  m1: HashMap<i32, i32>,
  m2: HashMap<i32, i32>,
  s2: Vec<i32>,
}

impl FindSumPairs {
  fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
    let mut m1: HashMap<i32, i32> = HashMap::new();
    let mut m2: HashMap<i32, i32> = HashMap::new();

    nums1.iter().for_each(|&v| {
      m1.entry(v).and_modify(|x| *x += 1).or_insert(1);
    });
    nums2.iter().for_each(|&v| {
      m2.entry(v).and_modify(|x| *x += 1).or_insert(1);
    });
    FindSumPairs {
      m1: m1,
      m2: m2,
      s2: nums2,
    }
  }

  fn add(&mut self, index: i32, val: i32) {
    let index = index as usize;
    self.m2.entry(self.s2[index]).and_modify(|x| *x -= 1);
    self.s2[index] += val;
    self
      .m2
      .entry(self.s2[index])
      .and_modify(|x| *x += 1)
      .or_insert(1);
  }

  fn count(&self, tot: i32) -> i32 {
    let mut ans: i32 = 0;
    for (k, v) in &self.m1 {
      let diff = tot - k;
      if self.m2.contains_key(&diff) {
        ans += self.m1.get(&k).unwrap() * self.m2.get(&diff).unwrap();
      }
    }

    ans
  }
}
