use std::collections::HashMap;
impl Solution {
  pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
    let mut m: HashMap<(i32, i32, i32), i64> = HashMap::new();
    m.insert((0, 0, 0), 1);
    m.insert((0, 0, 1), 1);
    m.insert((0, 1, 1), 1);
    m.insert((1, 0, 0), 1);

    for cnt in 2..=(zero + one) {
      for zcnt in 0..=cnt {
        let ocnt = cnt - zcnt;
        if zcnt > zero || ocnt > one {
          continue;
        }
        let vz = if zcnt > 0 {
          *m.entry((zcnt - 1, ocnt, 0)).or_insert(0) + *m.entry((zcnt - 1, ocnt, 1)).or_insert(0)
        } else {
          0
        } - if zcnt > limit {
          *m.entry((zcnt - limit - 1, ocnt, 1)).or_insert(0)
        } else {
          0
        };

        let vo = if ocnt > 0 {
          *m.entry((zcnt, ocnt - 1, 0)).or_insert(0) + *m.entry((zcnt, ocnt - 1, 1)).or_insert(0)
        } else {
          0
        } - if ocnt > limit {
          *m.entry((zcnt, ocnt - limit - 1, 0)).or_insert(0)
        } else {
          0
        };

        m.insert((zcnt, ocnt, 0), vz % 1000000007);
        m.insert((zcnt, ocnt, 1), vo % 1000000007);
      }
    }

    let ans =
      ((*m.get(&(zero, one, 0)).unwrap() + *m.get(&(zero, one, 1)).unwrap()) % 1000000007) as i32;
    if ans < 0 { ans + 1000000007 } else { ans }
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::number_of_stable_arrays(1, 1, 2));
  println!("{}", Solution::number_of_stable_arrays(1, 2, 1));
  println!("{}", Solution::number_of_stable_arrays(3, 3, 2));
  println!("{}", Solution::number_of_stable_arrays(14, 13, 59));
  println!("{}", Solution::number_of_stable_arrays(1, 3, 1));
  println!("{}", Solution::number_of_stable_arrays(1, 4, 2));
  println!("{}", Solution::number_of_stable_arrays(20, 15, 75));
  println!("{}", Solution::number_of_stable_arrays(31, 36, 60));
  println!("{}", Solution::number_of_stable_arrays(35, 35, 22));
  println!("{}", Solution::number_of_stable_arrays(76, 59, 99));
}
