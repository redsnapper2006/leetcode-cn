impl Solution {
  pub fn can_change(start: String, target: String) -> bool {
    let mut sidx: usize = 0;
    let mut tidx: usize = 0;
    let bbs: Vec<u8> = start.as_bytes().to_vec();
    let bbt: Vec<u8> = target.as_bytes().to_vec();

    while sidx < bbs.len() && tidx < bbt.len() {
      if bbs[sidx] == '_' as u8 {
        sidx += 1;
        continue;
      }
      if bbt[tidx] == '_' as u8 {
        tidx += 1;
        continue;
      }
      if bbs[sidx] != bbt[tidx] {
        return false;
      }
      if bbs[sidx] == 'L' as u8 && sidx < tidx {
        return false;
      }
      if bbs[sidx] == 'R' as u8 && sidx > tidx {
        return false;
      }

      sidx += 1;
      tidx += 1;
    }

    while sidx < bbs.len() {
      if bbs[sidx] != '_' as u8 {
        return false;
      }
      sidx += 1;
    }
    while tidx < bbt.len() {
      if bbt[tidx] != '_' as u8 {
        return false;
      }
      tidx += 1;
    }

    true
  }
}
