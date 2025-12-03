//9. Palindrome Number

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut temp_x : i32 = x;
        let mut rev_x : i32 = 0;
        while temp_x > 0 {
            rev_x *= 10;
            rev_x += temp_x % 10;
            temp_x /= 10; 
        }
        rev_x == x
    }
}
