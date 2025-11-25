impl Solution {
  pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
    let dict_vec = dictionary
      .iter()
      .map(|dict| dict.as_bytes().iter().map(|b| 1 << (b - b'a') as i32).collect::<Vec<i32>>())
      .collect::<Vec<Vec<i32>>>();

    let mut ans: Vec<String> = vec![];
    queries.iter().for_each(|q| {
      let offset = q.as_bytes().iter().map(|b| 1 << ((b - b'a') as i32)).collect::<Vec<i32>>();
      let mut found: bool = false;
      for dd in dict_vec.iter() {
        if offset
          .clone()
          .into_iter()
          .zip(dd.clone().into_iter())
          .collect::<Vec<(i32, i32)>>()
          .iter()
          .fold(0, |sum, (l, r)| sum + if l & r > 0 { 0 } else { 1 })
          <= 2
        {
          found = true;
          break;
        }
      }
      if found {
        ans.push(q.clone());
      }
    });
    ans
  }
}
