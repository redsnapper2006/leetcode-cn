impl Solution {
  pub fn count_good_numbers(n: i64) -> i32 {
    let mut ans: i64 = 1;
    let MOD : i64 = 1000000007;
    let mut m = n / 2;
    let mut base : i64 = 20;
    while m > 0 {
      if m & 1 == 1 {
        ans = (ans * base) % MOD;
      }
      base = (base * base) % MOD;
      m /= 2;
    }

    if n % 2 == 1 {
      ans = (ans * 5) % 1000000007;
    }
    ans as _
  }


  pub fn count_good_numbers2(n: i64) -> i32 {
    let table: Vec<i64> = vec![
      20, 400, 160000, 599999825, 480032116, 778421646, 719767538, 132124309, 906529657, 267965558,
      771612591, 422031379, 613868260, 997587941, 62347630, 939406318, 120927556, 697768825,
      733711322, 262661584, 226451307, 83046891, 56488379, 939711089, 608167453, 299433673,
      898647268, 630895482, 421808582, 603393378, 65665922, 281925993, 972660201, 986885388,
      230311210, 80361329, 153440640, 838801397, 684037163, 89740710, 974930342, 98812809,
      154122684, 557087249, 825964601, 329566582, 210863807, 791288441, 477248737, 373730684,
      185385495,
    ];
    let mut ans: i64 = 1;

    let mut m = n / 2;
    let mut idx: i64 = 50;
    while m > 0 {
      if m >= (1 << idx) {
        ans = (ans * table[idx as usize]) % 1000000007;
        m -= 1 << idx;
      } else {
        idx -= 1;
      }
    }

    if n % 2 == 1 {
      ans = (ans * 5) % 1000000007;
    }
    ans as _
  }
}

struct Solution {}

fn main() {
  // for i in 1..=10000 {
  //   println!("{}", Solution::count_good_numbers(i));
  // }
  // let mut tw: i64 = 20;
  // for i in 1..=50 {
  //   tw = (tw * tw) % 1000000007;
  //   println!("{}", tw);
  // }

  println!("{}", Solution::count_good_numbers(806166225460393));
}
