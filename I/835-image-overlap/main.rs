impl Solution {
  pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
    let i2v = |img: &[Vec<i32>]| -> Vec<i32> {
      img
        .iter()
        .map(|i| i.iter().fold(0, |sum, ii| sum * 2 + ii))
        .collect::<Vec<i32>>()
    };

    let n = img1.len();
    let mv = |img1: &[i32], img2: &[i32]| -> u32 {
      let mut ans: u32 = 0;
      for i in 0..n {
        let mut sum1: u32 = 0;
        let mut sum2: u32 = 0;
        for s in i..n {
          sum1 += (img1[s] & img2[s - i]).count_ones();
          sum2 += (img1[s - i] & img2[s]).count_ones();
        }
        ans = ans.max(sum1).max(sum2);
      }
      ans
    };

    let v1 = i2v(&img1);
    let v2 = i2v(&img2);

    let mut ans: u32 = 0;
    for i in 0..n {
      let t1 = v1.iter().map(|x| x << i).collect::<Vec<i32>>();
      let t2 = v1.iter().map(|x| x >> i).collect::<Vec<i32>>();

      ans = ans.max(mv(&t1, &v2)).max(mv(&t2, &v2));
    }
    ans as i32
  }
}
