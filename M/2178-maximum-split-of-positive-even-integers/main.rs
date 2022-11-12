struct Solution {}

impl Solution {
    pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
        let mut ret: Vec<i64> = Vec::new();
        if final_sum % 2 == 0 {
            let mut i: i64 = 0;
            let mut sum: i64 = 0;
            loop {
                i += 2;
                if sum + i > final_sum {
                    break;
                }
                sum += i;
                ret.push(i);
            }
            let offset = ret.len() - 1;
            ret[offset] += final_sum - sum;
        }

        ret
    }
}
