struct Solution {}

#[derive(PartialEq, Hash, Eq, Clone, Debug)]
pub struct Key {
  pub SM: u64,
  pub SE: i16,
  pub SS: i8,
  pub IM: u64,
  pub IE: i16,
  pub IS: i8,
  pub X: i32,
  pub T: bool,
}

use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;
impl Solution {
  pub fn best_line(points: Vec<Vec<i32>>) -> Vec<i32> {
    fn integer_decode(val: f64) -> (u64, i16, i8) {
      let bits: u64 = unsafe { mem::transmute(val) };
      let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
      let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
      let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
      } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
      };

      exponent -= 1023 + 52;
      (mantissa, exponent, sign)
    }

    let mut m: HashMap<Key, HashSet<usize>> = HashMap::new();

    (0..points.len()).for_each(|start| {
      (start + 1..points.len()).for_each(|next| {
        let mut key = if points[start][0] == points[next][0] {
          Key {
            SM: 0,
            SE: 0,
            SS: 0,
            IM: 0,
            IE: 0,
            IS: 0,
            X: points[start][0],
            T: true,
          }
        } else {
          let (sm, se, ss) = integer_decode(
            (points[start][1] - points[next][1]) as f64
              / (points[start][0] - points[next][0]) as f64,
          );
          let (im, ie, is) = integer_decode(
            points[start][1] as f64
              - ((points[start][1] - points[next][1]) as f64
                / (points[start][0] - points[next][0]) as f64)
                * points[start][0] as f64,
          );
          Key {
            SM: sm,
            SE: se,
            SS: ss,
            IM: im,
            IE: ie,
            IS: is,
            X: 0,
            T: false,
          }
        };
        let hs = m.entry(key).or_insert(HashSet::new());
        hs.insert(start);
        hs.insert(next);
        // println!("start {} next {} m {:?}", start, next, m);
      });
    });

    let mut vv = m
      .into_iter()
      .map(|(_, v)| {
        let mut vv = v.into_iter().collect::<Vec<usize>>();
        vv.sort();
        vv
      })
      .collect::<Vec<Vec<usize>>>();
    vv.sort_by(|v1, v2| {
      if v1.len() != v2.len() {
        return v2.len().cmp(&v1.len());
      }
      if v1[0] != v2[0] {
        return v1[0].cmp(&v2[0]);
      }
      v1[1].cmp(&v2[1])
    });
    vec![vv[0][0] as i32, vv[0][1] as i32]
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::best_line(vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![2, 0]])
  );
  println!(
    "{:?}",
    Solution::best_line(vec![
      vec![-38935, 27124],
      vec![-39837, 19604],
      vec![-7086, 42194],
      vec![-11571, -23257],
      vec![115, -23257],
      vec![20229, 5976],
      vec![24653, -18488],
      vec![11017, 21043],
      vec![-9353, 16550],
      vec![-47076, 15237],
      vec![-36686, 42194],
      vec![-17704, 1104],
      vec![31067, 7368],
      vec![-20882, 42194],
      vec![-19107, -10597],
      vec![-14898, 24506],
      vec![-20801, 42194],
      vec![-52268, 40727],
      vec![-14042, 42194],
      vec![-23254, 42194],
      vec![-30837, -53882],
      vec![1402, 801],
      vec![-33961, -984],
      vec![-6411, 42194],
      vec![-12210, 22901],
      vec![-8213, -19441],
      vec![-26939, 20810],
      vec![30656, -23257],
      vec![-27195, 21649],
      vec![-33780, 2717],
      vec![23617, 27018],
      vec![12266, 3608]
    ])
  );
}
