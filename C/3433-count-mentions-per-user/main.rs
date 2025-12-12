impl Solution {
  pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
    let mut events = events;
    events.sort_by(|x, y| {
      let x_ts = x[1].parse::<i32>().unwrap();
      let y_ts = y[1].parse::<i32>().unwrap();
      if x_ts != y_ts {
        x_ts.cmp(&y_ts)
      } else {
        y[0].cmp(&x[0])
      }
    });

    let nof = number_of_users as usize;
    let mut buf: Vec<(i32, i32)> = vec![(0, 0); nof];
    events.iter().for_each(|event| {
      let ts = event[1].parse::<i32>().unwrap();
      if event[0] == "MESSAGE" {
        let goal_ts = if event[2] == "ALL" { i32::MAX } else { ts };
        if event[2] == "ALL" || event[2] == "HERE" {
          for i in 0..buf.len() {
            if buf[i].1 <= goal_ts {
              buf[i].0 += 1;
            }
          }
        } else {
          for t in event[2]
            .split(' ')
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x[2..].parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
          {
            buf[t as usize].0 += 1;
          }
        }
      } else {
        buf[event[2].parse::<usize>().unwrap()].1 = ts + 60;
      }
    });
    buf.iter().map(|x| x.0).collect::<Vec<i32>>()
  }
}

struct Solution {}

fn main() {
  println!(
    "{:?}",
    Solution::count_mentions(
      2,
      vec![
        vec![
          "MESSAGE".to_string(),
          "10".to_string(),
          "id1 id0".to_string()
        ],
        vec!["OFFLINE".to_string(), "11".to_string(), "0".to_string()],
        vec!["MESSAGE".to_string(), "71".to_string(), "HERE".to_string()]
      ]
    )
  );
  println!(
    "{:?}",
    Solution::count_mentions(
      3,
      vec![
        vec!["MESSAGE".to_string(), "2".to_string(), "HERE".to_string()],
        vec!["OFFLINE".to_string(), "2".to_string(), "1".to_string()],
        vec!["OFFLINE".to_string(), "1".to_string(), "0".to_string()],
        vec!["MESSAGE".to_string(), "61".to_string(), "HERE".to_string()]
      ]
    )
  );
}
