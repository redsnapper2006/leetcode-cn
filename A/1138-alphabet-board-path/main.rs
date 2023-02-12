use std::collections::HashMap;
impl Solution {
  pub fn alphabet_board_path(target: String) -> String {
    let mut cord: HashMap<char, [usize; 2]> = HashMap::from([
      ('a', [0, 0]),
      ('b', [0, 1]),
      ('c', [0, 2]),
      ('d', [0, 3]),
      ('e', [0, 4]),
      ('f', [1, 0]),
      ('g', [1, 1]),
      ('h', [1, 2]),
      ('i', [1, 3]),
      ('j', [1, 4]),
      ('k', [2, 0]),
      ('l', [2, 1]),
      ('m', [2, 2]),
      ('n', [2, 3]),
      ('o', [2, 4]),
      ('p', [3, 0]),
      ('q', [3, 1]),
      ('r', [3, 2]),
      ('s', [3, 3]),
      ('t', [3, 4]),
      ('u', [4, 0]),
      ('v', [4, 1]),
      ('w', [4, 2]),
      ('x', [4, 3]),
      ('y', [4, 4]),
      ('z', [5, 0]),
    ]);

    let mut base: [usize; 2] = [0, 0];

    let mut ret: Vec<u8> = Vec::new();

    for b in target.chars() {
      let dst = cord.get(&b).unwrap();

      if b == 'z' {
        (dst[1]..base[1]).for_each(|_| {
          ret.push('L' as u8);
        });
        (base[0]..dst[0]).for_each(|_| {
          ret.push('D' as u8);
        });
      } else if base[0] == 5 && base[1] == 0 {
        (dst[0]..base[0]).for_each(|_| {
          ret.push('U' as u8);
        });
        (base[1]..dst[1]).for_each(|_| {
          ret.push('R' as u8);
        });
      } else {
        let mut h: u8 = 'L' as u8;
        let mut cs: usize = dst[1];

        let mut ce: usize = base[1];
        if base[1] < dst[1] {
          h = 'R' as u8;
          cs = base[1];
          ce = dst[1];
        }
        let mut v: u8 = 'U' as u8;
        let mut rs: usize = dst[0];

        let mut re: usize = base[0];

        if base[0] < dst[0] {
          v = 'D' as u8;
          rs = base[0];
          re = dst[0];
        }
        (rs..re).for_each(|_| {
          ret.push(v);
        });
        (cs..ce).for_each(|_| {
          ret.push(h);
        });
      }
      ret.push('!' as u8);
      base = *dst;
    }

    String::from_utf8(ret).unwrap()
  }
}
