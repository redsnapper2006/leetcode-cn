use std::collections::HashMap;
impl Solution {
  pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    let mut m: HashMap<i32, i32> = HashMap::new();
    arr.iter().for_each(|&v| {
      let mut v = v % k;
      if v < 0 {
        v += k;
      }
      let odd = (k - v) % k;
      if m.contains_key(&odd) {
        let cnt = m.get_mut(&odd).unwrap();
        *cnt -= 1;
        if *cnt == 0 {
          m.remove(&odd);
        }
      } else {
        m.entry(v % k).and_modify(|x| *x += 1).or_insert(1);
      }
    });

    m.len() == 0
  }
}
