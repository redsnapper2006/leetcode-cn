use std::collections::HashMap;

impl Solution {
  fn find(uf_map: &mut HashMap<i32, i32>, x: i32) -> i32 {
    let n = *uf_map.entry(x).or_insert(x);
    if n != x {
      let f = Self::find(uf_map, n);
      uf_map.insert(x, f);
      f
    } else {
      x
    }
  }
  fn union(uf_map: &mut HashMap<i32, i32>, x: i32, y: i32) {
    let xv = Self::find(uf_map, x);
    let yv = Self::find(uf_map, y);
    if xv != yv {
      uf_map.insert(xv, yv);
    }
  }

  pub fn minimum_hamming_distance(
    source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>,
  ) -> i32 {
    let mut source = source;
    let mut uf_map: HashMap<i32, i32> = HashMap::new();

    allowed_swaps.iter().for_each(|allow_swap| {
      Self::union(&mut uf_map, allow_swap[0], allow_swap[1]);
    });

    let mut groups: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..source.len() {
      let r = Self::find(&mut uf_map, i as i32);
      groups.entry(r).or_insert(vec![]).push(i as i32);
    }

    let mut ans: i32 = 0;
    groups.iter().for_each(|(_, group)| {
      let mut m: HashMap<i32, i32> = HashMap::new();
      for i in 0..group.len() {
        *m.entry(source[group[i] as usize]).or_insert(0) += 1;
        *m.entry(target[group[i] as usize]).or_insert(0) -= 1;
        source[group[i] as usize] = -1;
      }
      for (_, v) in m {
        ans += if v > 0 { v } else { 0 };
      }
    });

    ans
  }
}
