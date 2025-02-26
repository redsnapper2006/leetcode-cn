struct BrowserHistory {
  s: Vec<String>,
  o: usize,
}

impl BrowserHistory {
  fn new(homepage: String) -> Self {
    BrowserHistory {
      s: vec![homepage],
      o: 1,
    }
  }

  fn visit(&mut self, url: String) {
    if self.s.len() == self.o {
      self.s.push(url);
    } else {
      self.s[self.o] = url;
    }
    self.o += 1;
    self.s.resize(self.o, "".to_string());
  }

  fn back(&mut self, steps: i32) -> String {
    let diff = (self.o - 1).min(steps as usize);
    self.o -= diff;
    self.s[self.o - 1].clone()
  }

  fn forward(&mut self, steps: i32) -> String {
    let fw = self.s.len().min(self.o + steps as usize);
    self.o = fw;
    self.s[self.o - 1].clone()
  }
}

fn main() {
  let mut bh = BrowserHistory::new("leetcode.com".to_string());
  bh.visit("google.com".to_string());
  bh.visit("facebook.com".to_string());
  bh.visit("youtube.com".to_string());
  println!("back {}", bh.back(1));
  println!("back {}", bh.back(1));
  println!("forward {}", bh.forward(1));
  bh.visit("linkedin.com".to_string());
  println!("forward {}", bh.forward(2));
  println!("back {}", bh.back(2));
  println!("back {}", bh.back(7));

  bh = BrowserHistory::new("esgriv.com".to_string());
  bh.visit("cgrt.com".to_string());
  bh.visit("tip.com".to_string());
  println!("back {}", bh.back(9));
  bh.visit("kttzxgh.com".to_string());
  println!("forward {}", bh.forward(7));
  bh.visit("crqje.com".to_string());
  bh.visit("iybch.com".to_string());
  println!("forward {}", bh.forward(5));
  bh.visit("uun.com".to_string());
  println!("back {}", bh.back(10));
  bh.visit("hci.com".to_string());
  bh.visit("whula.com".to_string());
  println!("forward {}", bh.forward(10));
}
