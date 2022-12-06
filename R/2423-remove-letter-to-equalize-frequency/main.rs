struct Solution {}

impl Solution {
  pub fn equal_frequency(word: String) -> bool {
    let mut cnt: Vec<i32> = vec![0; 26];
    word.as_bytes().iter().for_each(|w| {
      cnt[(w - 'a' as u8) as usize] += 1;
    });

    let mut pure = cnt.into_iter().filter(|x| *x != 0).collect::<Vec<i32>>();
    pure.sort();
    let k1 = pure[0];
    let p1 = pure
      .clone()
      .into_iter()
      .filter(|x| *x == k1)
      .collect::<Vec<i32>>();
    let p2 = pure
      .clone()
      .into_iter()
      .filter(|x| *x != k1)
      .collect::<Vec<i32>>();
    if p2.len() == 0 {
      return k1 == 1 || p1.len() == 1;
    }
    let k2 = p2[0];
    let p3 = p2
      .clone()
      .into_iter()
      .filter(|x| *x != k2)
      .collect::<Vec<i32>>();
    //  println!("{:?} {:?} {:?} {:?} {:?}  ", pk3, p2, pk1, k2 ,k1);

    if p3.len() > 0 {
      return false;
    }
    if p1.len() > 1 && p2.len() > 1 {
      return false;
    }
    if p1.len() > 1 && p2.len() == 1 {
      return (k1-k2).abs() == 1;
    }
    if p1.len() == 1 && p2.len() > 1 {
      return k1 == 1;
    }

    k1 == 1 || k2 == 1 || (k1-k2).abs() == 1
  }
}

fn main() {
  println!("{}", Solution::equal_frequency("abcc".to_string()));
}
