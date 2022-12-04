struct Solution {}

impl Solution {
  pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for gar in &garbage {
      sum += gar.len() as i32;
    }

    let mut m: usize = 0;
    let mut p: usize = 0;
    let mut g: usize = 0;
    for i in (1..garbage.len()).rev() {
      for gar in garbage[i].as_bytes() {
        if *gar == 'M' as u8 && m == 0 {
          m = i;
        }
        if *gar == 'P' as u8 && p == 0 {
          p = i;
        }
        if *gar == 'G' as u8 && g == 0 {
          g = i;
        }
      }
    }
    let mut aggr: Vec<i32> = vec![0];
    let mut s: i32 = 0;
    for t in travel {
      s += t;
      aggr.push(s);
    }
    sum + aggr[m] + aggr[p] + aggr[g]
  }
}
