impl Solution {
  pub fn parse_bool_expr(expression: String) -> bool {
    let mut v_stack: Vec<bool> = vec![];
    let mut o_stack: Vec<(u8, usize)> = vec![];

    let bb = expression.as_bytes().to_vec();
    let mut idx: usize = 0;
    while idx < bb.len() {
      if bb[idx] == b',' {
        idx += 1;
        continue;
      }

      if bb[idx] == b'!' || bb[idx] == b'&' || bb[idx] == b'|' {
        o_stack.push((bb[idx], v_stack.len()));
        idx += 1;
      } else if bb[idx] == b'f' || bb[idx] == b't' {
        v_stack.push(if bb[idx] == b't' { true } else { false });
      } else {
        let mut v: bool = v_stack.pop().unwrap();
        let (o, idx2) = o_stack.pop().unwrap();
        if o == b'!' {
          v = !v;
        } else {
          while v_stack.len() > idx2 {
            let n = v_stack.pop().unwrap();
            if o == b'&' {
              v = v & n;
            } else {
              v = v | n;
            }
          }
        }
        v_stack.push(v);
      }
      idx += 1;
    }
    v_stack[0]
  }
}
