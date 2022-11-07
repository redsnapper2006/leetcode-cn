struct Solution {}

impl Solution {
    pub fn sub(s: Vec<u8>) -> Vec<String> {
        if s.len() == 0 {
            return Vec::new();
        }
        if s.len() == 1 {
            return vec![String::from_utf8(s).unwrap()];
        }
        let (mut left, mut right, mut times): (i32, i32, i32) = (0, 0, 1);
        for i in 0..s.len() {
            // println!("{} {} {}", s[i as usize], '0' as u8, times);
            right = right * 10 + (s[i as usize] - '0' as u8) as i32;
            times = times * 10;
        }
        let mut ret: Vec<String> = Vec::new();
        for i in 0..s.len() {
            let offset = (s[i as usize] - '0' as u8) as i32;
            left = left * 10 + offset;
            right = right - offset * times;
            times = times / 10;

            if i > 0 && s[0] == '0' as u8 {
                continue;
            }
            if i < s.len() - 1 && s[s.len() - 1] == '0' as u8 {
                continue;
            }

            if left == 0 && i > 0 {
                continue;
            }
            if right == 0 && i < s.len() - 1 {
                continue;
            }

            if i == s.len() - 1 {
                ret.push(String::from_utf8(s.clone()).unwrap());
            } else {
                ret.push(
                    String::from_utf8(s[0..i + 1].to_vec()).unwrap()
                        + "."
                        + &String::from_utf8(s[i + 1..].to_vec()).unwrap(),
                );
            }
        }
        ret
    }
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let trim = s.as_bytes();
        let bb = trim[1..trim.len() - 1].to_vec();
        println!("{:?}", bb);
        let mut ret: Vec<String> = Vec::new();
        for i in 0..bb.len() {
            let (left, right) = (
                Solution::sub(bb[0..i].to_vec()),
                Solution::sub(bb[i..].to_vec()),
            );
            for m in 0..left.len() {
                for n in 0..right.len() {
                    let t: Vec<String> = vec![
                        String::from("("),
                        left[m].clone(),
                        String::from(", "),
                        right[n].clone(),
                        String::from(")"),
                    ];

                    ret.push(t.join(""));
                }
            }
        }
        ret
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::ambiguous_coordinates(String::from("(100)"))
    )
}
