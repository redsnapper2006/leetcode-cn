use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
  pub fn max_amount(
    initial_currency: String,
    pairs1: Vec<Vec<String>>,
    rates1: Vec<f64>,
    pairs2: Vec<Vec<String>>,
    rates2: Vec<f64>,
  ) -> f64 {
    let mut m1: HashMap<String, f64> = HashMap::new();
    let mut q1: VecDeque<(String, f64)> = VecDeque::new();
    q1.push_back((initial_currency.clone(), 1.0));
    let mut pm: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    for i in 0..pairs1.len() {
      let p1 = &pairs1[i][0];
      let p2 = &pairs1[i][1];
      pm.entry(p1.clone())
        .and_modify(|x| x.push((p2.clone(), rates1[i])))
        .or_insert(vec![(p2.clone(), rates1[i])]);
      pm.entry(p2.clone())
        .and_modify(|x| x.push((p1.clone(), 1.0 / rates1[i])))
        .or_insert(vec![(p1.clone(), 1.0 / rates1[i])]);
    }
    while q1.len() > 0 {
      let (cur, cnt) = q1.pop_front().unwrap();
      if !pm.contains_key(&cur) {
        continue;
      }
      for nxt in pm.get(&cur).unwrap() {
        let (nxt_cur, nxt_cnt) = nxt;
        if !m1.contains_key(nxt_cur) || *m1.get(nxt_cur).unwrap() < nxt_cnt * cnt {
          m1.insert(nxt_cur.clone(), nxt_cnt * cnt);
          q1.push_back((nxt_cur.clone(), nxt_cnt * cnt));
        }
      }
    }

    let mut m2: HashMap<String, f64> = HashMap::new();
    let mut q2: VecDeque<(String, f64)> = VecDeque::new();
    for (k, v) in m1.iter() {
      q2.push_back((k.clone(), v.clone()));
    }
    let mut pm2: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    for i in 0..pairs2.len() {
      let p1 = &pairs2[i][0];
      let p2 = &pairs2[i][1];
      pm2
        .entry(p1.clone())
        .and_modify(|x| x.push((p2.clone(), rates2[i])))
        .or_insert(vec![(p2.clone(), rates2[i])]);
      pm2
        .entry(p2.clone())
        .and_modify(|x| x.push((p1.clone(), 1.0 / rates2[i])))
        .or_insert(vec![(p1.clone(), 1.0 / rates2[i])]);
    }
    while q2.len() > 0 {
      let (cur, cnt) = q2.pop_front().unwrap();
      if !pm2.contains_key(&cur) {
        continue;
      }
      for nxt in pm2.get(&cur).unwrap() {
        let (nxt_cur, nxt_cnt) = nxt;
        if !m2.contains_key(nxt_cur) || *m2.get(nxt_cur).unwrap() < nxt_cnt * cnt {
          m2.insert(nxt_cur.clone(), nxt_cnt * cnt);
          q2.push_back((nxt_cur.clone(), nxt_cnt * cnt));
        }
      }
    }
    if !m2.contains_key(&initial_currency) {
      1.0
    } else {
      *m2.get(&initial_currency).unwrap()
    }
  }
}
