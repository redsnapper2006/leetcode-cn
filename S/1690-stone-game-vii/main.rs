struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    for i in 0..stones.len() {
      total += stones[i];
    }

    let mut m: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    fn dfs(
      steps: i32,
      left: i32,
      right: i32,
      stones: &Vec<i32>,
      remain: i32,
      m: &mut HashMap<(i32, i32), (i32, i32)>,
    ) -> (i32, i32) {
      if left > right {
        return (0, 0);
      }

      if let Some(&v) = m.get(&(left, right)) {
        return v;
      }

      let mut a1 = 0;
      let mut b1 = 0;
      if steps % 2 == 0 {
        a1 = remain - stones[left as usize];
      } else {
        b1 = remain - stones[left as usize];
      }
      let (ra1, rb1) = dfs(
        steps + 1,
        left + 1,
        right,
        stones,
        remain - stones[left as usize],
        m,
      );

      let mut a2 = 0;
      let mut b2 = 0;
      if steps % 2 == 0 {
        a2 = remain - stones[right as usize];
      } else {
        b2 = remain - stones[right as usize];
      }
      let (ra2, rb2) = dfs(
        steps + 1,
        left,
        right - 1,
        stones,
        remain - stones[right as usize],
        m,
      );

      match (a1 + ra1 - b1 - rb1 > a2 + ra2 - b2 - rb2, steps % 2 == 0) {
        (true, true) => {
          m.insert((left, right), (a1 + ra1, b1 + rb1));
          (a1 + ra1, b1 + rb1)
        }
        (true, false) => {
          m.insert((left, right), (a2 + ra2, b2 + rb2));
          (a2 + ra2, b2 + rb2)
        }
        (false, true) => {
          m.insert((left, right), (a2 + ra2, b2 + rb2));
          (a2 + ra2, b2 + rb2)
        }
        (_, _) => {
          m.insert((left, right), (a1 + ra1, b1 + rb1));
          (a1 + ra1, b1 + rb1)
        }
      }
    }

    let (a, b) = dfs(0, 0, stones.len() as i32 - 1, &stones, total, &mut m);
    a - b
  }
}

fn main() {
  println!("{}", Solution::stone_game_vii(vec![5, 3, 1, 4, 2]));
  println!(
    "{}",
    Solution::stone_game_vii(vec![7, 90, 5, 1, 100, 10, 10, 2])
  );
}
