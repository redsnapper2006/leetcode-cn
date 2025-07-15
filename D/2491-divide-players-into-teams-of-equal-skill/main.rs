use std::collections::HashMap;
impl Solution {
  pub fn divide_players(skill: Vec<i32>) -> i64 {
    if skill.len() % 2 == 1 {
      return -1;
    }

    let g = skill.len() as i64 / 2;
    let mut sum: i64 = 0;
    let mut m: HashMap<i64, i64> = HashMap::new();
    skill.iter().for_each(|&v| {
      let v = v as i64;
      sum += v;
      m.entry(v).and_modify(|x| *x += 1).or_insert(1);
    });

    if sum % g != 0 {
      return -1;
    }
    let gs = sum / g;
    let mut ans: i64 = 0;
    for v in &skill {
      let v = *v as i64;
      let diff = gs - v;
      if *m.get(&v).unwrap() == 0 {
        continue;
      }

      m.entry(v).and_modify(|x| *x -= 1);
      if !m.contains_key(&diff) || *m.get(&diff).unwrap() == 0 {
        return -1;
      }
      m.entry(diff).and_modify(|x| *x -= 1);
      ans += diff * v;
    }
    ans
  }
}
