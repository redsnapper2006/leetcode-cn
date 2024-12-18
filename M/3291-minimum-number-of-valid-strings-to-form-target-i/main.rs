struct Solution {}

impl Solution {
  pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
    let ww = words
      .iter()
      .map(|word| word.as_bytes().to_vec())
      .collect::<Vec<Vec<u8>>>();
    let tt = target.as_bytes().to_vec();
    let mut dp: Vec<i32> = vec![-1; tt.len() + 1];
    dp[tt.len()] = 0;
    (0..tt.len()).rev().for_each(|idx| {
      let mut min: i32 = tt.len() as i32 + 1;
      ww.iter().for_each(|www| {
        let mut ii: usize = 0;
        while ii < www.len() && ii + idx < tt.len() {
          if www[ii] != tt[ii + idx] {
            break;
          }
          if dp[ii + idx + 1] != -1 {
            min = min.min(dp[ii + idx + 1]);
          }
          ii += 1;
        }
      });
      if min != target.len() as i32 + 1 {
        dp[idx] = min + 1;
      }
    });

    dp[0]
  }

  pub fn min_valid_strings2(words: Vec<String>, target: String) -> i32 {
    fn dfs(words: &Vec<Vec<u8>>, target: &Vec<u8>, idx: usize, dp: &mut Vec<i32>) -> i32 {
      if idx == target.len() {
        return 0;
      }

      if dp[idx] != target.len() as i32 + 1 {
        return dp[idx];
      }
      let mut min: i32 = target.len() as i32 + 1;
      words.iter().for_each(|word| {
        let mut ii: usize = 0;
        while ii < word.len() && idx + ii < target.len() {
          if word[ii] != target[idx + ii] {
            break;
          }
          let v = dfs(words, target, idx + ii + 1, dp);
          if v != -1 {
            min = min.min(v);
          }
          ii += 1;
        }
      });
      if min != target.len() as i32 + 1 {
        dp[idx] = min + 1;
      } else {
        dp[idx] = -1
      }
      dp[idx]
    }

    let ww = words
      .iter()
      .map(|word| word.as_bytes().to_vec())
      .collect::<Vec<Vec<u8>>>();
    let tt = target.as_bytes().to_vec();
    let mut dp: Vec<i32> = vec![target.len() as i32 + 1; target.len()];
    dfs(&ww, &tt, 0, &mut dp)
  }
}

fn main() {
  println!(
    "{}",
    Solution::min_valid_strings(
      vec!["abc".to_string(), "aaaaa".to_string(), "bcdef".to_string()],
      "aabcdabc".to_string()
    )
  );
  println!(
    "{}",
    Solution::min_valid_strings(
      vec!["abababab".to_string(), "ab".to_string()],
      "ababaababa".to_string()
    )
  );
  println!(
    "{}",
    Solution::min_valid_strings(
      vec!["dbjaibdafjijjcihdhccbbaeggaceeaadieaae".to_string(),
      "efiiggg".to_string(),
      "gbcfdhaffe".to_string(),"gchgiig".to_string(),"jeeeebiegifjbhaeghgbfihgjd".to_string(),"bhcbagjjjc".to_string(),"becce".to_string(),"jdbejgfjahbheh".to_string(),"e".to_string(),"hccfgdjggaeadbchbiidaccaheihfcjiggdfaggbgdhajdaeeegdehejidacfdajjdjjjeacddfggeadcbdbciijhbciaejfgaefafggdfdafajghceghggcbaihadgiejddjcajhdagchbfafahedddcbficfhdgjjfdcaafajcgggjidifchaaeididheeafdabiajbdhjciaaefeabgffeefhhgdbggajjbbffcehfbfgdhghiffagejebhdicieieigjhhdcaibhcjifafiidchijhhjgddciegjigcibhiaghcfihcabdhifidbdhbacfhgcehgiehhjjjaiehajgeeddcdiagcaaigdjgbdcaffeedbffaijbhhhffcbjjadgaebcfhdgchdgahffhadcgghbiaecieceggdceagbedhjichgibdjifaahffaggdgbbfefagchejdidghcebcicbdaacfdfdegibfihhigecgjehfbghcejacfejfedhidbccchgihheibbbcdbggghciggihbbfeadjaabgfbfgjbajgdejddhfheehhfjhddjihhfd".to_string(),"ceef".to_string(),"feibehchhhffiibjaahcjgaeg".to_string(),"bdcebf".to_string(),"gibibide".to_string(),"ghjiiefdegiibecbf".to_string(),"cchjhjjebcbhjahajchchfdeihbejhaccbchhhjh".to_string(),"gdhjfahcdhb".to_string(),"ghhdiiaeiadegafe".to_string(),"dia".to_string(),"cgcejfcaeeahifhaadggigjedjbcidhchficgabi".to_string()],
      "cgcefbbcehgjcbcgggdbddifddchgfgjabjehbehdfcdggicijejajgibhjahdaicajhffjgeaddeiichafgbddieacacijajbbh".to_string()
    )
  );
}
