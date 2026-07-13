impl Solution {
  pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![];
    for c in ((low.ilog10() + 1)..=(high.ilog10() + 1)) {
      let (mut base, mut step): (i32, i32) =
        (1..=c as i32).fold((0, 0), |(base, step), v| (base * 10 + v, step * 10 + 1));
      while base <= high {
        if base % 10 == 0 {
          break;
        }
        if base >= low {
          ans.push(base);
        }
        base += step;
      }
    }

    ans
  }

  pub fn sequential_digits2(low: i32, high: i32) -> Vec<i32> {
    let mut cnt: i32 = 0;
    let mut ll = low;
    while ll > 0 {
      ll /= 10;
      cnt += 1;
    }

    let mut first: i32 = 1;
    let mut ans: Vec<i32> = vec![];
    while true {
      let mut b = first;
      let mut t: i32 = 0;
      for _ in 0..cnt {
        t = t * 10 + b;
        b += 1;
      }

      if t >= low && t <= high {
        ans.push(t);
      } else if t > high {
        break;
      }

      if first + cnt - 1 == 9 {
        first = 0;
        cnt += 1;
      }
      first += 1;
    }
    ans
  }
}
