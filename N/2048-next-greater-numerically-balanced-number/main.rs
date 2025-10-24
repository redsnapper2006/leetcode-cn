impl Solution {
  #[allow(dead_code)]
  pub fn gen_nbn() {
    for i in 1..1230000 {
      let mut m = i;
      let mut digits = [0; 10];
      let mut is_match: bool = true;
      while m > 0 {
        let mode = m % 10;
        m /= 10;
        digits[mode as usize] += 1;
        if digits[mode as usize] > mode {
          break;
        }
      }
      (0..10).for_each(|i| {
        if digits[i] != 0 && digits[i] != i as i32 {
          is_match = false;
        }
      });

      if is_match {
        println!("{}", i);
      }
    }
  }

  pub fn next_beautiful_number(n: i32) -> i32 {
    let table: Vec<i32> = vec![
      1, 22, 122, 212, 221, 333, 1333, 3133, 3313, 3331, 4444, 14444, 22333,
      23233, 23323, 23332, 32233, 32323, 32332, 33223, 33232, 33322, 41444,
      44144, 44414, 44441, 55555, 122333, 123233, 123323, 123332, 132233,
      132323, 132332, 133223, 133232, 133322, 155555, 212333, 213233, 213323,
      213332, 221333, 223133, 223313, 223331, 224444, 231233, 231323, 231332,
      232133, 232313, 232331, 233123, 233132, 233213, 233231, 233312, 233321,
      242444, 244244, 244424, 244442, 312233, 312323, 312332, 313223, 313232,
      313322, 321233, 321323, 321332, 322133, 322313, 322331, 323123, 323132,
      323213, 323231, 323312, 323321, 331223, 331232, 331322, 332123, 332132,
      332213, 332231, 332312, 332321, 333122, 333212, 333221, 422444, 424244,
      424424, 424442, 442244, 442424, 442442, 444224, 444242, 444422, 515555,
      551555, 555155, 555515, 555551, 666666, 1224444,
    ];

    table[match table.binary_search(&n) {
      Ok(ov) => ov + 1,
      Err(ev) => ev,
    }]
  }

  #[allow(dead_code)]
  pub fn next_beautiful_number2(n: i32) -> i32 {
    let mut n = n + 1;

    loop {
      let mut m = n;
      let mut digits = [0; 10];

      let mut is_match: bool = true;
      while m > 0 {
        let mode = m % 10;
        m /= 10;
        digits[mode as usize] += 1;
        if digits[mode as usize] > mode {
          break;
        }
      }
      (0..10).for_each(|i| {
        if digits[i] != 0 && digits[i] != i as i32 {
          is_match = false;
        }
      });
      if is_match {
        break;
      }

      n += 1;
    }
    n
  }
}

struct Solution {}

fn main() {
  // Solution::gen_nbn();
  println!("{}", Solution::next_beautiful_number(1));
  println!("{}", Solution::next_beautiful_number(1000));
  println!("{}", Solution::next_beautiful_number(3000));
}
