struct Solution {}

impl Solution {
  pub fn give_gem(gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
    let mut gem = gem;
    operations.iter().for_each(|oper| {
      let v = gem[oper[0] as usize] / 2;
      gem[oper[0] as usize] -= v;
      gem[oper[1] as usize] += v;
    });
    gem.sort();
    gem[gem.len() - 1] - gem[0]
  }
}
