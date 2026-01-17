impl Solution {
  pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let ws = words
      .iter()
      .map(|word| {
        let mut s: (Vec<i32>, i32) = (vec![0; 26], 0);
        word.as_bytes().iter().for_each(|&b| {
          let offset = (b - b'a') as usize;
          s.0[offset] += 1;
          s.1 += score[offset];
        });
        s
      })
      .collect::<Vec<(Vec<i32>, i32)>>();

    let count = letters.iter().fold(vec![0; 26], |mut sum, &l| {
      sum[l as usize - 'a' as usize] += 1;
      sum
    });

    fn dfs(
      ws: &Vec<(Vec<i32>, i32)>, idx: usize, count: &Vec<i32>, sum: &mut Vec<i32>, score: &mut i32,
      ans: &mut i32,
    ) {
      if idx == ws.len() {
        *ans = *ans.max(score);
        return;
      }

      for i in idx..ws.len() {
        for j in 0..26 {
          sum[j] += ws[i].0[j];
        }
        *score += ws[i].1;

        let mut valid: bool = true;
        for j in 0..26 {
          if sum[j] > count[j] {
            valid = false;
            break;
          }
        }
        if valid {
          dfs(ws, i + 1, count, sum, score, ans);
        }
        for j in 0..26 {
          sum[j] -= ws[i].0[j];
        }
        *score -= ws[i].1;
      }
      *ans = *ans.max(score);
    }

    let mut sum: Vec<i32> = vec![0; 26];
    let mut score: i32 = 0;
    let mut ans: i32 = 0;
    dfs(&ws, 0, &count, &mut sum, &mut score, &mut ans);
    ans
  }
}
