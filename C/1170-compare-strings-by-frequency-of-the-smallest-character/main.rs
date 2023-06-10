struct Solution {}

impl Solution {
  pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
    fn count(word: &String) -> i32 {
      let mut cnt: i32 = 0;
      let mut max: u8 = 255;
      word.as_bytes().to_vec().iter().for_each(|&b| {
        if max > b {
          max = b;
          cnt = 1;
        } else if max == b {
          cnt += 1;
        }
      });
      cnt
    }
    let mut cnt = words.iter().map(|word| count(word)).collect::<Vec<i32>>();
    cnt.sort();

    queries
      .iter()
      .map(|query| {
        let c = count(query);
        let mut start: i32 = 0;
        let mut end: i32 = cnt.len() as i32 - 1;
        while start <= end {
          let mid = start + (end - start) / 2;
          if cnt[mid as usize] > c {
            end = mid - 1;
          } else {
            start = mid + 1;
          }
        }
        cnt.len() as i32 - start
      })
      .collect::<Vec<i32>>()
  }
}
