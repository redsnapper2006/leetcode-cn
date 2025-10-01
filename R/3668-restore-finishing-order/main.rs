impl Solution {
  pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
    let mut buf: Vec<i32> = vec![0; 101];
    friends.iter().for_each(|&f| {
      buf[f as usize] = 1;
    });
    order.iter().fold(vec![], |mut aggr, &o| {
      if buf[o as usize] == 1 {
        aggr.push(o);
      }
      aggr
    })
  }
}
