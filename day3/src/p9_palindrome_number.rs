pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut z = x;
        let mut y = 0;

        // x = 123, y = 0
        // y = 3, x = 12 
        // y = 32, x = 1 
        // y = 321, x = 0

        while z > 0 {
            y = z % 10 + y * 10;
            z /= 10;
        }
        
        x == y
    }
}

#[test]
fn test() {
    assert_eq!(true, Solution::is_palindrome(121));
    assert_eq!(true, Solution::is_palindrome(1221));
    assert_eq!(true, Solution::is_palindrome(12121));
    assert_eq!(false, Solution::is_palindrome(-12121));
    assert_eq!(false, Solution::is_palindrome(-1234));
}


