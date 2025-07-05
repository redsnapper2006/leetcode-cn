use std::collections::HashMap;
impl Solution {
  pub fn find_lucky(arr: Vec<i32>) -> i32 {
    let mut m : HashMap<i32,i32>  = HashMap::new();

    arr.iter().for_each(|&v| {
      m.entry(v).and_modify(|x| *x+=1).or_insert(1);
    });
    let mut ans : i32 = -1;
    for (k, v) in m {
      if v == k {
        ans = ans.max(k);
      }
    }
    ans
  }
}
