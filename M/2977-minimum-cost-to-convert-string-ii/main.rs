use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
  pub fn minimum_cost(
    source: String, target: String, original: Vec<String>, changed: Vec<String>, cost: Vec<i32>,
  ) -> i64 {
    let mut length: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; 2]; source.len()];
    original.iter().for_each(|org| {
      for i in 0..=source.len() - org.len() {
        if source[i..i + org.len()] == *org.as_str() {
          length[i][0].push(org.len());
        }
      }
    });
    changed.iter().for_each(|chg| {
      for i in 0..=target.len() - chg.len() {
        if target[i..i + chg.len()] == *chg.as_str() {
          length[i][1].push(chg.len());
        }
      }
    });
    for i in 0..length.len() {
      length[i][0].sort_unstable();
      length[i][1].sort_unstable();
    }
    let orig_set: HashSet<&str> = HashSet::from_iter(original.iter().map(|s| s as &str));
    let chg_set: HashSet<&str> = HashSet::from_iter(changed.iter().map(|s| s as &str));
    let mut edge_set: HashSet<&str> = HashSet::from_iter(original.iter().map(|s| s as &str));
    for s in changed.iter() {
      edge_set.insert(s.as_str());
    }
    let mut em: HashMap<&str, usize> = HashMap::new();
    edge_set.iter().enumerate().for_each(|(i, s)| {
      em.insert(s, i);
    });
    let mut ocm: Vec<Vec<i64>> = vec![vec![i64::MAX; edge_set.len()]; edge_set.len()];
    for i in 0..edge_set.len() {
      ocm[i][i] = 0;
    }
    for i in 0..original.len() {
      let s = *em.get(original[i].as_str()).unwrap();
      let d = *em.get(changed[i].as_str()).unwrap();
      ocm[s][d] = ocm[s][d].min(cost[i] as i64);
    }
    for b in 0..edge_set.len() {
      for s in 0..edge_set.len() {
        for d in 0..edge_set.len() {
          if b == s || b == d || ocm[s][b] == i64::MAX || ocm[b][d] == i64::MAX {
            continue;
          }
          ocm[s][d] = ocm[s][d].min(ocm[s][b] + ocm[b][d]);
        }
      }
    }

    let mut cache: Vec<(i64, bool)> = vec![(i64::MAX, false); source.len()];
    fn dfs(
      s: &String, t: &String, idx: usize, orig_set: &HashSet<&str>, chg_set: &HashSet<&str>,
      em: &HashMap<&str, usize>, length: &Vec<Vec<Vec<usize>>>, ocm: &Vec<Vec<i64>>,
      cache: &mut Vec<(i64, bool)>,
    ) -> i64 {
      if idx == s.len() {
        return 0;
      }
      if cache[idx].1 {
        return cache[idx].0;
      }
      let mut mnv: i64 = i64::MAX;
      if s[idx..idx + 1] == t[idx..idx + 1] {
        mnv = mnv.min(dfs(s, t, idx + 1, orig_set, chg_set, em, length, ocm, cache));
      }

      let slen = &length[idx][0];
      let clen = &length[idx][1];
      let mut sidx: usize = 0;
      let mut cidx: usize = 0;
      while sidx < slen.len() && cidx < clen.len() {
        if slen[sidx] < clen[cidx] {
          sidx += 1;
        } else if slen[sidx] > clen[cidx] {
          cidx += 1;
        } else {
          if orig_set.contains(&s[idx..idx + slen[sidx]])
            && chg_set.contains(&t[idx..idx + clen[cidx]])
          {
            let v = ocm[*em.get(&s[idx..idx + slen[sidx]]).unwrap()]
              [*em.get(&t[idx..idx + clen[cidx]]).unwrap()];
            if v != i64::MAX {
              let nv = dfs(s, t, idx + slen[sidx], orig_set, chg_set, em, length, ocm, cache);
              if nv != i64::MAX {
                mnv = mnv.min(v + nv);
              }
            }
          }
          sidx += 1;
          cidx += 1;
        }
      }

      cache[idx] = (cache[idx].0.min(mnv), true);
      mnv
    }

    dfs(&source, &target, 0, &orig_set, &chg_set, &em, &length, &ocm, &mut cache);
    if cache[0].0 == i64::MAX {
      -1
    } else {
      cache[0].0 as i64
    }
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::minimum_cost(
      "abcd".to_string(),
      "acbe".to_string(),
      vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "c".to_string(),
        "e".to_string(),
        "d".to_string()
      ],
      vec![
        "b".to_string(),
        "c".to_string(),
        "b".to_string(),
        "e".to_string(),
        "b".to_string(),
        "e".to_string()
      ],
      vec![2, 5, 5, 1, 2, 20]
    )
  );

  println!(
    "{}",
    Solution::minimum_cost(
      "abcdefgh".to_string(),
      "acdeeghh".to_string(),
      vec!["bcd".to_string(), "fgh".to_string(), "thh".to_string()],
      vec!["cde".to_string(), "thh".to_string(), "ghh".to_string(),],
      vec![1, 3, 5]
    )
  );

  println!(
    "{}",
    Solution::minimum_cost(
      "abcdefgh".to_string(),
      "addddddd".to_string(),
      vec!["bcd".to_string(), "defgh".to_string()],
      vec!["ddd".to_string(), "ddddd".to_string()],
      vec![100, 1578]
    )
  );
}
