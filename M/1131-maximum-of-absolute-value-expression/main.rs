impl Solution {
  pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut a: Vec<i32> = vec![];
    let mut b: Vec<i32> = vec![];
    let mut c: Vec<i32> = vec![];
    let mut d: Vec<i32> = vec![];
    for i in 0..arr1.len() {
      a.push(arr1[i] + arr2[i] + i as i32);
      b.push(arr1[i] + arr2[i] - i as i32);
      c.push(arr1[i] - arr2[i] + i as i32);
      d.push(arr1[i] - arr2[i] - i as i32);
    }
    (a.iter().max().unwrap() - a.iter().min().unwrap())
      .max(b.iter().max().unwrap() - b.iter().min().unwrap())
      .max(c.iter().max().unwrap() - c.iter().min().unwrap())
      .max(d.iter().max().unwrap() - d.iter().min().unwrap())
  }
}
