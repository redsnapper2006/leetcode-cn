impl Solution {
  pub fn concat_hex36(n: i32) -> String {
    let nn = n * n;
    let mut buf : Vec<u8> = vec![];
    let mut n16 = nn;
    while n16 > 0 {
      let e = n16 % 16;
      if e < 10 {
        buf.push(b'0' + e as u8 );
      } else {
        buf.push(b'A' + (e - 10) as u8);
      }
      n16/=16;
    }
    let mut start : usize = 0;
    let mut end : usize = buf.len() -1;
    while start < end {
      let t = buf[start];
      buf[start]= buf[end];
      buf[end] = t;
      start +=1;
      end -=1;
    }
    start = buf.len();
    let mut n36 = n*n*n;
    while n36 > 0 {
      let e = n36 % 36;
      if e < 10 {
        buf.push(b'0' + e as u8 );
      } else {
        buf.push(b'A' + (e - 10) as u8);
      }

      n36/=36;
    }
    end  = buf.len() -1;
    while start < end {
      let t = buf[start];
      buf[start]= buf[end];
      buf[end] = t;
      start +=1;
      end -=1;
    }
    String::from_utf8(buf).unwrap()
  }
}
