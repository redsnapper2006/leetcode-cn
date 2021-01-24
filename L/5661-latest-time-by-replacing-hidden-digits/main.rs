struct Solution {}
impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut bytes = time.into_bytes();
        if bytes[0] == b'?' {
            if bytes[1] == b'?' || bytes[1] < b'4' {
                bytes[0] = b'2';
            } else {
                bytes[0] = b'1';
            }
        }
        if bytes[1] == b'?' {
            if bytes[0] == b'2' {
                bytes[1] = b'3'
            } else {
                bytes[1] = b'9'
            }
        }
        if bytes[3] == b'?' {
            bytes[3] = b'5';
        }
        if bytes[4] == b'?' {
            bytes[4] = b'9';
        }

        String::from_utf8(bytes).unwrap()
    }
}

fn main() {
    println!("{}", Solution::maximum_time(String::from("2?:?0")));
}
