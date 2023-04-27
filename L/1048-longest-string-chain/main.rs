struct Solution {}

impl Solution {
  pub fn longest_str_chain(words: Vec<String>) -> i32 {
    let mut w = words;
    w.sort_by(|x, y| {
      if x.len() != y.len() {
        return x.len().cmp(&y.len());
      }
      x.cmp(&y)
    });
    // println!("{:?}", w);
    let bw: Vec<Vec<u8>> = w
      .iter()
      .map(|v| v.as_bytes().to_vec())
      .collect::<Vec<Vec<u8>>>();

    let mut dp: Vec<i32> = vec![1; w.len()];
    (1..bw.len()).for_each(|idx| {
      let mut res: i32 = 1;
      (0..idx).for_each(|pidx| {
        if bw[pidx].len() + 1 != bw[idx].len() {
          return;
        }
        let mut diff: i32 = 0;
        let mut po: usize = 0;
        let mut o: usize = 0;
        while po < bw[pidx].len() && o < bw[idx].len() {
          if bw[pidx][po] == bw[idx][o] {
            po += 1;
            o += 1;
          } else {
            diff += 1;
            if diff >= 2 {
              break;
            }
            o += 1;
          }
        }
        // println!("pidx {} idx {} {} {} ", pidx, idx, po, o);
        if po == bw[pidx].len() && o >= bw[idx].len() - 1 && dp[pidx] + 1 > res {
          res = dp[pidx] + 1;
        }
      });
      dp[idx] = res;
      // println!("idx {} dp {:?}", idx, dp);
    });

    *dp.iter().max().unwrap()
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::longest_str_chain(vec![
      "ba".to_string(),
      "bca".to_string(),
      "bda".to_string(),
      "bdca".to_string(),
      "a".to_string(),
      "b".to_string(),
    ])
  );

  println!(
    "{:?}",
    Solution::longest_str_chain(vec![
      "xbc".to_string(),
      "pcxbcf".to_string(),
      "xb".to_string(),
      "cxbc".to_string(),
      "pcxbc".to_string()
    ])
  );

  println!(
    "{:?}",
    Solution::longest_str_chain(vec!["abcd".to_string(), "dbqca".to_string()])
  );
}
