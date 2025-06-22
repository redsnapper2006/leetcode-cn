use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
  pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
    let mut m: HashMap<i32, i32> = HashMap::new();
    let prim: HashSet<i32> = HashSet::from_iter(
      vec![
        00002, 00003, 00005, 00007, 00011, 00013, 00017, 00019, 00023, 00029, 00031, 00037, 00041,
        00043, 00047, 00053, 00059, 00061, 00067, 00071, 00073, 00079, 00083, 00089, 00097,
      ]
      .iter()
      .cloned(),
    );
    nums.iter().for_each(|&v| {
      m.entry(v).and_modify(|x| *x += 1).or_insert(1);
    });

    for (_, v) in m {
      if prim.contains(&v) {
        return true;
      }
    }
    false
  }
}
