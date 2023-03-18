struct Solution {}

impl Solution {
  pub fn check_palindrome_formation(a: String, b: String) -> bool {
    let mut start: usize = 0;
    let mut end: usize = a.len() - 1;

    while start < end {
      if a.as_bytes()[start] == b.as_bytes()[end] {
        start += 1;
        end -= 1;
      } else {
        break;
      }
    }
    let mut ns = start;
    let mut ne = end;
    while ns < ne {
      if b.as_bytes()[ns] == b.as_bytes()[ne] {
        ns += 1;
        ne -= 1;
      } else {
        break;
      }
    }
    if ns >= ne {
      return true;
    }
    ns = start;
    ne = end;
    while ns < ne {
      if a.as_bytes()[ns] == a.as_bytes()[ne] {
        ns += 1;
        ne -= 1;
      } else {
        break;
      }
    }
    if ns >= ne {
      return true;
    }
    start = 0;
    end = b.len() - 1;
    while start < end {
      if b.as_bytes()[start] == a.as_bytes()[end] {
        start += 1;
        end -= 1;
      } else {
        break;
      }
    }
    ns = start;
    ne = end;
    while ns < ne {
      if b.as_bytes()[ns] == b.as_bytes()[ne] {
        ns += 1;
        ne -= 1;
      } else {
        break;
      }
    }
    if ns >= ne {
      return true;
    }
    ns = start;
    ne = end;
    while ns < ne {
      if a.as_bytes()[ns] == a.as_bytes()[ne] {
        ns += 1;
        ne -= 1;
      } else {
        break;
      }
    }
    if ns >= ne {
      return true;
    }
    false
  }
}
