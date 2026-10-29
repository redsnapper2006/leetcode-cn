struct BusBooking {
  seats: Vec<i8>,
  cost_time: Vec<Vec<i32>>,
  total_time: i32,
  min_time: i32,
}

impl BusBooking {
  fn new(n: i32) -> Self {
    BusBooking {
      seats: vec![0; n as usize + 1],
      cost_time: vec![vec![0; 4]; n as usize + 1],
      total_time: 0i32,
      min_time: 0i32,
    }
  }

  fn toggle(&mut self, seat: String) {
    let offset = (seat.as_bytes()[0] - b'A') as usize;
    let idx = seat.get(1..).unwrap().parse::<usize>().unwrap();
    if self.seats[idx] & (1 << offset) > 0 {
      self.total_time -= self.cost_time[idx][offset];
      self.min_time -= 1;
      self.cost_time[idx][offset] = 0;
      self.seats[idx] -= 1 << offset;
    } else {
      self.cost_time[idx][offset] = 1;
      if offset == 0 && self.seats[idx] & 2 != 0 || offset == 3 && self.seats[idx] & 4 != 0 {
        self.cost_time[idx][offset] += 2;
      }
      self.total_time += self.cost_time[idx][offset];
      self.min_time += 1;
      self.seats[idx] |= 1 << offset;
    }
  }

  fn get_total_time(&self) -> i32 {
    self.total_time
  }

  fn get_min_time(&self) -> i32 {
    self.min_time
  }
}
