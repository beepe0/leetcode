struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for id in 0..nums.len(){
            for i in 0..nums.len() {
                if id != i && target == nums[id] + nums[i] {
                    return vec![id as i32, i as i32];
                }
            }
        }
        
        vec![]
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![3,2,3], 6))
}