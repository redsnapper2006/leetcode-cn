impl Solution {
  pub fn maximum_gap(skill: String, station: String) -> i32 {
    let skill_bb = skill.as_bytes().to_vec();
    let station_bb = station.as_bytes().to_vec();
    let mut buf: Vec<usize> = vec![0; skill.len()];

    let mut station_idx: usize = 0;
    let mut gap: usize = 0;
    for i in 0..skill_bb.len() {
      while station_bb[station_idx] != skill_bb[i] {
        station_idx += 1;
      }
      buf[i] = station_idx;
      if i > 0 {
        gap = gap.max(buf[i] - buf[i - 1]);
      }
      station_idx += 1;
    }

    let mut station_idx: usize = station_bb.len() - 1;
    for i in (1..skill_bb.len()).rev() {
      while station_bb[station_idx] != skill_bb[i] {
        station_idx -= 1;
      }
      buf[i] = station_idx;
      gap = gap.max(buf[i] - buf[i - 1]);
      station_idx -= 1;
    }
    gap as i32
  }
}
