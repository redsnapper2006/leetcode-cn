impl Solution {
  pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
    words.iter().enumerate().fold(-1, |ans, (idx, word)| {
      if *word != target {
        ans
      } else {
        let off = (start_index - idx as i32).abs();
        if ans == -1 {
          off.min(words.len() as i32 - off)
        } else {
          ans.min(off).min(words.len() as i32 - off)
        }
      }
    })
  }

  pub fn closest_target2(words: Vec<String>, target: String, start_index: i32) -> i32 {
    let mut offset: Vec<usize> = Vec::new();
    let size = words.len();
    words.iter().enumerate().for_each(|(idx, v)| {
      if *v == target {
        offset.push(idx);
      }
    });

    if offset.len() == 0 {
      return -1;
    }

    let mut min: i32 = size as i32 + 1;
    offset.iter().for_each(|&off| {
      let mut diff: i32 = off as i32 - start_index;
      if diff < 0 {
        diff = -diff;
      }
      if min > diff {
        min = diff;
      }
      diff = size as i32 - diff;
      if min > diff {
        min = diff;
      }
    });
    min
  }
}

struct Solution {}
