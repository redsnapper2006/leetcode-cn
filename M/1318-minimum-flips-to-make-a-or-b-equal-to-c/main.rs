struct Solution {}

impl Solution {
  pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    let mut va: Vec<i32> = Vec::new();
    let mut vb: Vec<i32> = Vec::new();
    let mut vc: Vec<i32> = Vec::new();

    let mut ba = a;
    let mut bb = b;
    let mut bc = c;

    (0..32).for_each(|_| {
      va.push(ba % 2);
      vb.push(bb % 2);
      vc.push(bc % 2);
      ba /= 2;
      bb /= 2;
      bc /= 2;
    });

    (0..32)
      .map(|idx| match (vc[idx], va[idx], vb[idx]) {
        (0, a, b) => a + b,
        (1, 0, 0) => 1,
        (_, _, _) => 0,
      })
      .sum()
  }
}

fn main() {
  println!("{}", Solution::min_flips(1, 2, 3));
}
