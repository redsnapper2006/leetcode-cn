struct MyCalendar {
  s: Vec<i32>,
  e: Vec<i32>,
}

impl MyCalendar {
  fn new() -> Self {
    MyCalendar {
      s: vec![],
      e: vec![],
    }
  }

  fn book(&mut self, start: i32, end: i32) -> bool {
    let so = match self.s.binary_search(&start) {
      Ok(_) => -1,
      Err(ev) => ev as i32,
    };

    let eo = match self.e.binary_search(&end) {
      Ok(_) => -1,
      Err(ev) => ev as i32,
    };
    if so == -1 || eo == -1 || so != eo {
      return false;
    }

    let so = so as usize;
    if so > 0 && start < self.e[so - 1] || so < self.e.len() && end > self.s[so] {
      return false;
    }

    self.e.insert(so, end);
    self.s.insert(so, start);
    true
  }
}

fn main() {
  let mut mc = MyCalendar::new();

  println!("{}", mc.book(20, 29));
  println!("{}", mc.book(13, 22));
  println!("{}", mc.book(44, 50));
  println!("{}", mc.book(1, 7));
  println!("{}", mc.book(2, 10));
  println!("{}", mc.book(14, 20));
  println!("{}", mc.book(19, 25));
  println!("{}", mc.book(36, 42));
  println!("{}", mc.book(45, 50));
  println!("{}", mc.book(47, 50));
  println!("{}", mc.book(39, 45));
  println!("{}", mc.book(44, 50));
  println!("{}", mc.book(16, 25));
  println!("{}", mc.book(45, 50));
  println!("{}", mc.book(45, 50));
  println!("{}", mc.book(12, 20));
  println!("{}", mc.book(21, 29));
  println!("{}", mc.book(11, 20));
  println!("{}", mc.book(12, 17));
  println!("{}", mc.book(34, 40));
  println!("{}", mc.book(10, 18));
  println!("{}", mc.book(38, 44));
  println!("{}", mc.book(23, 32));
  println!("{}", mc.book(38, 44));
  println!("{}", mc.book(15, 20));
  println!("{}", mc.book(27, 33));
  println!("{}", mc.book(34, 42));
  println!("{}", mc.book(44, 50));
  println!("{}", mc.book(35, 40));
  println!("{}", mc.book(24, 31));
}
