use std::collections::BTreeMap;
use std::collections::HashSet;
use std::ops::Bound::Excluded;
impl Solution {
  pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
    let mut area: BTreeMap<i64, i64> = BTreeMap::new();
    let mut bottom: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    let mut top: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    let mut total: f64 = 0.0;
    squares.iter().enumerate().for_each(|(idx, square)| {
      let b = square[1] as i64;
      let l = square[2] as i64;
      *area.entry(b + l).or_insert(0) += l * l;
      total += l as f64 * l as f64;
      bottom.entry(b).or_insert(vec![]).push(idx);
      top.entry(b + l).or_insert(vec![]).push(idx);
    });
    let mut buf: Vec<(i64, i64)> = vec![];
    let mut s: f64 = 0.0;
    let mut e: f64 = 0.0;
    for (&k, &v) in area.iter() {
      buf.push((k, v));
      if buf.len() > 1 {
        let s = buf.len();
        buf[s - 1].1 += buf[s - 2].1;
      }
      e = e.max(k as f64);
    }

    while s <= e {
      let m = s + (e - s) / 2.0;
      let mm = m as i64;
      let ii = match buf.binary_search_by(|b| b.0.cmp(&mm)) {
        Ok(v) => v as i32,
        Err(v) => v as i32 - 1,
      };

      let mut t = if ii >= 0 {
        buf[ii as usize].1 as f64
      } else {
        0.0
      };
      let mut b_idx: HashSet<usize> = HashSet::new();
      for (_, value) in bottom.range((Excluded(&i64::MIN), Excluded(&(mm + 1)))) {
        value.iter().for_each(|&v| {
          b_idx.insert(v);
        });
      }
      let mut t_idx: HashSet<usize> = HashSet::new();
      for (_, value) in top.range((Excluded(&mm), Excluded(&i64::MAX))) {
        value.iter().for_each(|&v| {
          t_idx.insert(v);
        });
      }
      for x in b_idx.intersection(&t_idx) {
        let square = &squares[*x];
        let b = square[1] as f64;
        let l = square[2] as f64;
        t += (m - b) * l;
      }
      if t >= (total / 2.0) {
        e = m - 0.00001;
      } else {
        s = m + 0.00001;
      }
    }

    s as f64
  }
}

struct Solution {}
fn main() {
  println!(
    "{}",
    Solution::separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]])
  );

  println!(
    "{}",
    Solution::separate_squares(vec![vec![0, 0, 2], vec![1, 1, 1]])
  );

  println!(
    "{}",
    Solution::separate_squares(vec![
      vec![522261215, 954313664, 225462],
      vec![628661372, 718610752, 10667],
      vec![619734768, 941310679, 44788],
      vec![352367502, 656774918, 289036],
      vec![860247066, 905800565, 100123],
      vec![817623994, 962847576, 71460],
      vec![691552058, 782740602, 36271],
      vec![911356, 152015365, 513881],
      vec![462847044, 859151855, 233567],
      vec![672324240, 954509294, 685569]
    ])
  );
}
