struct Solution {}

impl Solution {
  pub fn count_substrings(s: String, t: String) -> i32 {
    let sb = s.as_bytes();
    let tb = t.as_bytes();

    let mut dpl: Vec<Vec<i32>> = vec![vec![0; t.len() + 1]; s.len() + 1];
    let mut dpr: Vec<Vec<i32>> = vec![vec![0; t.len() + 1]; s.len() + 1];
    for sidx in 0..sb.len() {
      for tidx in 0..tb.len() {
        if sb[sidx] == tb[tidx] {
          dpl[sidx + 1][tidx + 1] = dpl[sidx][tidx] + 1;
        }
      }
    }
    for sidx in (0..sb.len()).rev() {
      for tidx in (0..tb.len()).rev() {
        if sb[sidx] == tb[tidx] {
          dpr[sidx][tidx] = dpr[sidx + 1][tidx + 1] + 1;
        }
      }
    }

    let mut ret: i32 = 0;
    for sidx in 0..sb.len() {
      for tidx in 0..tb.len() {
        if sb[sidx] != tb[tidx] {
          ret += (dpl[sidx][tidx] + 1) * (dpr[sidx + 1][tidx + 1] + 1);
        }
      }
    }

    ret
  }

  #[allow(dead_code)]
  pub fn count_substrings2(s: String, t: String) -> i32 {
    let sb = s.as_bytes();
    let tb = t.as_bytes();

    let mut ret: i32 = 0;
    for sidx in 0..sb.len() {
      for tidx in 0..tb.len() {
        if sb[sidx] == tb[tidx] {
          continue;
        }

        let mut rs = sidx + 1;
        let mut rt = tidx + 1;
        while rs < sb.len() && rt < tb.len() {
          if sb[rs] != tb[rt] {
            break;
          }
          rs += 1;
          rt += 1;
        }

        let mut ls: i32 = sidx as i32 - 1;
        let mut lt: i32 = tidx as i32 - 1;
        while ls >= 0 && lt >= 0 {
          if sb[ls as usize] != tb[lt as usize] {
            break;
          }
          ls -= 1;
          lt -= 1;
        }

        // println!("{} {} {} {} {} {}", sidx, tidx, rs, rt, ls, lt);
        ret += (rs as i32 - sidx as i32) * (sidx as i32 - ls as i32)
      }
    }
    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::count_substrings("abe".to_string(), "bbc".to_string())
  );

  println!(
    "{}",
    Solution::count_substrings("aba".to_string(), "baba".to_string())
  );

  println!(
    "{}",
    Solution::count_substrings("ab".to_string(), "bb".to_string())
  );

  println!(
    "{}",
    Solution::count_substrings("a".to_string(), "a".to_string())
  );
}
