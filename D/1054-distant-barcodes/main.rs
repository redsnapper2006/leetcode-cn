struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
    let mut m: HashMap<i32, i32> = HashMap::new();
    barcodes.iter().for_each(|&v| {
      m.entry(v).and_modify(|x| *x += 1).or_insert(1);
    });

    let mut cnt: Vec<(i32, i32)> = Vec::new();
    m.iter().for_each(|(&k, &v)| {
      cnt.push((k, v));
    });

    cnt.sort_by(|x, y| {
      let xc = &x.1;
      let yc = &y.1;
      let xv = &x.0;
      let yv = &y.0;
      if xc != yc {
        return yc.cmp(xc);
      }
      xv.cmp(yv)
    });

    let mut buf: Vec<i32> = Vec::new();
    cnt.iter().for_each(|(v, c)| {
      (0..*c).for_each(|_| {
        buf.push(*v);
      });
    });
    let offset: usize = (barcodes.len() + 1) / 2;
    let mut res: Vec<i32> = Vec::new();
    (0..offset).for_each(|idx| {
      res.push(buf[idx]);
      if offset + idx < buf.len() {
        res.push(buf[offset + idx]);
      }
    });

    res
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::rearrange_barcodes(vec![1, 1, 1, 1, 2, 2, 3, 3])
  );

  println!("{:?}", Solution::rearrange_barcodes(vec![1, 1, 1, 2, 2, 2]));
}
