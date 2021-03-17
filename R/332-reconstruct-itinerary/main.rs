struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn recur(
    map: &HashMap<String, Vec<String>>,
    l: &mut Vec<String>,
    visit: &mut Vec<String>,
    visitflag: &mut Vec<bool>,
    steps: usize,
  ) -> Vec<String> {
    // println!("recur start : {:?}", l);
    if l.len() == steps {
      return l.to_vec();
    }
    let candi = l[l.len() - 1].clone();
    if !map.contains_key(&candi) {
      return l.to_vec();
    }
    for i in 0..map[&candi].len() {
      let next = map[&candi][i].clone();
      let mut v = candi.clone();
      v.push_str(&next);
      let mut idx: i32 = -1;
      for j in 0..visit.len() {
        if visit[j] != v {
          continue;
        }
        if visitflag[j] == true {
          continue;
        }
        idx = j as i32;
        break;
      }
      if idx == -1 {
        continue;
      }

      l.push(next);
      visitflag[idx as usize] = true;
      let ret = Solution::recur(map, l, visit, visitflag, steps);
      if ret.len() == steps {
        return ret;
      }
      l.remove(l.len() - 1);
      visitflag[idx as usize] = false;
    }
    l.to_vec()
  }

  pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut fmap: HashMap<String, Vec<String>> = HashMap::new();
    let mut visit: Vec<String> = vec![String::from(""); tickets.len()];
    let mut visitflag: Vec<bool> = vec![false; tickets.len()];
    for i in 0..tickets.len() {
      let candi = tickets[i].clone();
      let t = fmap.entry(candi[0].clone()).or_insert(Vec::new());
      t.push(candi[1].clone());

      let mut c = candi[0].clone();
      c.push_str(&(candi[1].clone()));
      visit[i] = c;
    }
    for (_k, v) in &mut fmap {
      v.sort();
    }
    // println!("{:?}", fmap);
    let mut l: Vec<String> = vec![String::from("JFK")];
    Solution::recur(&fmap, &mut l, &mut visit, &mut visitflag, tickets.len() + 1)
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::find_itinerary(vec![
      vec![String::from("EZE"), String::from("AXA")],
      vec![String::from("TIA"), String::from("ANU")],
      vec![String::from("ANU"), String::from("JFK")],
      vec![String::from("JFK"), String::from("ANU")],
      vec![String::from("ANU"), String::from("EZE")],
      vec![String::from("TIA"), String::from("ANU")],
      vec![String::from("AXA"), String::from("TIA")],
      vec![String::from("TIA"), String::from("JFK")],
      vec![String::from("ANU"), String::from("TIA")],
      vec![String::from("JFK"), String::from("TIA")]
    ])
  );

  //   println!(
  //     "{:?}",
  //     Solution::find_itinerary(vec![
  //       vec![String::from("MUC"), String::from("LHR")],
  //       vec![String::from("JFK"), String::from("MUC")],
  //       vec![String::from("SFO"), String::from("SJC")],
  //       vec![String::from("LHR"), String::from("SFO")]
  //     ])
  //   );

  //   println!(
  //     "{:?}",
  //     Solution::find_itinerary(vec![
  //       vec![String::from("JFK"), String::from("SFO")],
  //       vec![String::from("JFK"), String::from("ATL")],
  //       vec![String::from("SFO"), String::from("ATL")],
  //       vec![String::from("ATL"), String::from("JFK")],
  //       vec![String::from("ATL"), String::from("SFO")]
  //     ])
  //   );
}
