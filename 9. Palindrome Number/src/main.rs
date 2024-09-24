struct Solution;

fn reverse_number(mut n: i32) -> i32 {
    let mut reversed = 0;

    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }

    reversed
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        return x - reverse_number(x) == 0;
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(-121))
}

