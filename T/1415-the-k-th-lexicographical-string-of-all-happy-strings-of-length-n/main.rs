impl Solution {
  pub fn get_happy_string(n: i32, k: i32) -> String {
    if k > 3 * (1 << (n - 1)) {
      return "".to_string();
    }

    let base: Vec<u8> = vec![b'a', b'b', b'c'];
    let mut k = k - 1;
    let mut ans: Vec<u8> = vec![base[(k / (1 << (n - 1))) as usize]];
    k = k % (1 << (n - 1));
    let mut n = n - 1;
    while n > 0 {
      let base = match ans[ans.len() - 1] {
        b'a' => vec![b'b', b'c'],
        b'b' => vec![b'a', b'c'],
        _ => vec![b'a', b'b'],
      };
      ans.push(base[(k / (1 << (n - 1))) as usize]);
      k %= (1 << (n - 1));
      n -= 1;
    }

    String::from_utf8(ans).unwrap()
  }

  pub fn get_happy_string2(n: i32, k: i32) -> String {
    fn dfs(n: i32, buf: &mut Vec<u8>, stack: &mut Vec<Vec<u8>>) {
      if buf.len() as i32 == n {
        stack.push(buf.clone());
        return;
      }

      for b in vec![b'a', b'b', b'c'] {
        if buf.len() != 0 && buf[buf.len() - 1] == b {
          continue;
        }
        buf.push(b);
        dfs(n, buf, stack);
        buf.pop();
      }
    }

    let mut stack: Vec<Vec<u8>> = vec![];
    let mut buf: Vec<u8> = vec![];
    dfs(n, &mut buf, &mut stack);

    if (stack.len() as i32) < k {
      "".to_string()
    } else {
      String::from_utf8(stack[k as usize - 1].clone()).unwrap()
    }
  }
}
