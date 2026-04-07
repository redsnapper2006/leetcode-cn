struct Robot {
  width: i32,
  height: i32,
  step: i32,
}

impl Robot {
  fn new(width: i32, height: i32) -> Self {
    Robot {
      width: width,
      height: height,
      step: 0,
    }
  }

  fn step(&mut self, num: i32) {
    self.step = (self.step + num - 1) % (self.width * 2 + self.height * 2 - 4) + 1;
  }

  fn get_pos(&self) -> Vec<i32> {
    if self.step < self.width {
      vec![self.step, 0]
    } else if self.step < self.width + self.height - 1 {
      vec![self.width - 1, self.step - self.width + 1]
    } else if self.step < self.width * 2 + self.height - 2 {
      vec![
        self.width * 2 + self.height - self.step - 3,
        self.height - 1,
      ]
    } else {
      vec![0, self.width * 2 + self.height * 2 - self.step - 4]
    }
  }

  fn get_dir(&self) -> String {
    if self.step < self.width {
      "East".to_string()
    } else if self.step < self.width + self.height - 1 {
      "North".to_string()
    } else if self.step < self.width * 2 + self.height - 2 {
      "West".to_string()
    } else {
      "South".to_string()
    }
  }
}

struct Robot2 {
  width: i32,
  height: i32,
  x: i32,
  y: i32,
  d: i32, // E: 0, N:1, W:2, S:3
}

impl Robot2 {
  fn new(width: i32, height: i32) -> Self {
    Robot {
      width: width,
      height: height,
      x: 0,
      y: 0,
      d: 0,
    }
  }

  fn step(&mut self, num: i32) {
    let N = self.width * 2 + self.height * 2 - 4;
    let mut n = num % N;
    while n > 0 {
      if self.d == 0 {
        let s = (self.width - 1 - self.x).min(n);
        self.x += s;
        n -= s;
        if self.x == self.width - 1 {
          self.d = 1;
        }
      }
      if self.d == 1 {
        let s = (self.height - 1 - self.y).min(n);
        self.y += s;
        n -= s;
        if self.y == self.height - 1 {
          self.d = 2;
        }
      }
      if self.d == 2 {
        let s = self.x.min(n);
        self.x -= s;
        n -= s;
        if self.x == 0 {
          self.d = 3;
        }
      }
      if self.d == 3 {
        let s = (self.y).min(n);
        self.y -= s;
        n -= s;
        if self.y == 0 {
          self.d = 0;
        }
      }
    }

    if self.d == 0 && self.x == 0 {
      self.d = 3;
    } else if self.d == 1 && self.y == 0 {
      self.d = 0;
    } else if self.d == 2 && self.x == self.width - 1 {
      self.d = 1;
    } else if self.d == 3 && self.y == self.height - 1 {
      self.d = 2;
    }
  }

  fn get_pos(&self) -> Vec<i32> {
    vec![self.x, self.y]
  }

  fn get_dir(&self) -> String {
    match self.d {
      0 => "East".to_string(),
      1 => "North".to_string(),
      2 => "West".to_string(),
      _ => "South".to_string(),
    }
  }
}
