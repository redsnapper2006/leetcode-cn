struct Solution {}
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut sum: i32 = 0;
        let mut max: i32 = 0;
        for x in &gain {
            sum += x;
            if sum > 0 && sum > max {
                max = sum;
            }
        }
        max
    }
}

fn main() {
    println!("{}", Solution::largest_altitude([-5, 1, 5, 0, -7].to_vec()));
}
