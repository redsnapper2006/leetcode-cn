impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
      let mut buf : Vec<i32> = vec![0; grid.len()*grid.len()];

      grid.iter().for_each(| row| {
        row.iter().for_each(|&val| {
          buf[val as usize-1] +=1;
        });
      });

      let mut missing: i32 = 0;
      let mut duplicate : i32 = 0;
      buf.iter().enumerate().for_each(|(idx, val)| {
        if *val == 0 {
          missing = idx as i32+1;
        } else if *val == 2 {
          duplicate = idx as i32+1;
        }
      });
      vec![duplicate, missing]
    }
}
