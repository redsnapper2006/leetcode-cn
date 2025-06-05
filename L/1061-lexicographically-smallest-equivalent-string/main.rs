impl Solution {
  pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
    let mut dp: Vec<Vec<u8>> = vec![vec![]; 26];
    let b1 = s1.as_bytes().to_vec();
    let b2 = s2.as_bytes().to_vec();

    for i in 0..b1.len() {
      dp[(b1[i] - b'a') as usize].push(b2[i]);
      dp[(b2[i] - b'a') as usize].push(b1[i]);
    }

    let mut m : Vec<u8> = vec![0;26];
    (0..26 as u8).for_each(|idx| {
      if dp[idx as usize].len() == 0 {
        m[idx as usize] = b'a' + idx;
      }
    });

    fn dfs(dp: &Vec<Vec<u8>>, m: &mut Vec<u8>, visit: &mut Vec<bool>, idx: usize) -> u8 {
      if dp[idx as usize].len() == 0 {
        return m[idx];
      }

      visit[idx] = true;
      let mut ans : u8 = b'a' + idx as u8;
      for next in &dp[idx] {
        if visit[(next - b'a') as usize] {
          continue;
        }
        ans = ans.min(dfs(dp, m, visit, (next - b'a') as usize));
      }

      ans
    }

    (0..26).for_each(|idx| {
      let mut visit: Vec<bool> = vec![false; 26];
      let ans =  dfs(&mut dp, &mut m, &mut visit, idx);
      m[idx] = ans;
    });


    String::from_utf8(
      base_str
        .as_bytes()
        .to_vec()
        .iter()
        .map(|&b| m[(b - b'a') as usize])
        .collect::<Vec<u8>>(),
    )
    .unwrap()
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::smallest_equivalent_string(
      "parker".to_string(),
      "morris".to_string(),
      "parser".to_string()
    )
  );
  println!(
    "{}",
    Solution::smallest_equivalent_string(
      "hello".to_string(),
      "world".to_string(),
      "hold".to_string()
    )
  );
  println!(
    "{}",
    Solution::smallest_equivalent_string(
      "leetcode".to_string(),
      "programs".to_string(),
      "sourcecode".to_string()
   //  aauccacada
  //   aauaaaaada
    )
  );

}
