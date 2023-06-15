struct Solution {}

impl Solution {
  pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let bb: Vec<u8> = s.as_bytes().to_vec();

    let mut buf: Vec<[i32; 26]> = vec![[0; 26]; bb.len()];
    bb.iter().enumerate().for_each(|(idx, b)| {
      if idx > 0 {
        (0..26).for_each(|ii| {
          buf[idx][ii] = buf[idx - 1][ii];
        });
      }
      let offset = (b - 'a' as u8) as usize;
      buf[idx][offset] += 1;
    });

    queries
      .iter()
      .map(|range| {
        let left = range[0];
        let right = range[1];
        let k = range[2];

        let mut diff = buf[right as usize];
        if left > 0 {
          (0..26).for_each(|idx| {
            diff[idx] -= buf[left as usize - 1][idx];
          });
        }

        let mut cnt: i32 = 0;
        (0..26).for_each(|idx| {
          cnt += diff[idx] % 2;
        });
        // println!("{:?} {} {} ",diff ,cnt, k);
        cnt <= k * 2 + 1
      })
      .collect::<Vec<bool>>()
  }
}
