impl Solution {
  pub fn validate_coupons(
    code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>,
  ) -> Vec<String> {
    let mut cbi_zip = code
      .into_iter()
      .zip(business_line.into_iter().zip(is_active.into_iter()))
      .filter(|(c, (b, i))| {
        *i && (b == "electronics" || b == "grocery" || b == "pharmacy" || b == "restaurant")
          && c.len() > 0
          && c.as_bytes().iter().all(|&b| {
            b >= b'a' && b <= b'z' || b >= b'A' && b <= b'Z' || b >= b'0' && b <= b'9' || b == b'_'
          })
      })
      .collect::<Vec<(String, (String, bool))>>();
    cbi_zip.sort_by(|x, y| {
      if x.1.0 != y.1.0 {
        x.1.0.cmp(&y.1.0)
      } else {
        x.0.cmp(&y.0)
      }
    });
    cbi_zip.into_iter().map(|(c, _)| c).collect::<Vec<String>>()
  }

  pub fn validate_coupons2(
    code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>,
  ) -> Vec<String> {
    let mut e: Vec<String> = vec![];
    let mut g: Vec<String> = vec![];
    let mut p: Vec<String> = vec![];
    let mut r: Vec<String> = vec![];

    for i in 0..code.len() {
      if !is_active[i] {
        continue;
      }
      let bb = code[i].as_bytes().to_vec();
      if bb.len() == 0 {
        continue;
      }

      let mut valid: bool = true;
      for j in 0..bb.len() {
        if !(bb[j] >= b'a' && bb[j] <= b'z'
          || bb[j] >= b'A' && bb[j] <= b'Z'
          || bb[j] >= b'0' && bb[j] <= b'9'
          || bb[j] == b'_')
        {
          valid = false;
          break;
        }
      }
      if valid {
        match business_line[i].as_str() {
          "electronics" => e.push(code[i].clone()),
          "grocery" => g.push(code[i].clone()),
          "pharmacy" => p.push(code[i].clone()),
          "restaurant" => r.push(code[i].clone()),
          _ => (),
        };
      }
    }
    e.sort_unstable();
    g.sort_unstable();
    p.sort_unstable();
    r.sort_unstable();
    let mut ans: Vec<String> = vec![];
    for s in e {
      ans.push(s);
    }
    for s in g {
      ans.push(s);
    }
    for s in p {
      ans.push(s);
    }
    for s in r {
      ans.push(s);
    }
    ans
  }
}
