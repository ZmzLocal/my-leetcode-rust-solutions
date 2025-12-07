//5. Longest Palindromic Substring

impl Solution {
    fn expand_palindrom(svec: &Vec<char>, mut left: i32, mut right: i32) -> (i32, i32) {
        while left >= 0 && right < svec.len() as i32 && svec[left as usize] == svec[right as usize] {
            left -= 1;
            right += 1;
        }
        (left + 1, right - 1)
    }

    pub fn longest_palindrome(s: String) -> String {
        let letters: Vec<char> = s.chars().collect();
        let mut start: i32 = 0;
        let mut end: i32 = 0;

        for i in 0..letters.len() {
            let i = i as i32;
            let (l1, r1) = Solution::expand_palindrom(&letters, i, i);
            let (l2, r2) = Solution::expand_palindrom(&letters, i, i + 1);

            if r1 - l1 > end - start {
                start = l1;
                end = r1;
            }
            if r2 - l2 > end - start {
                start = l2;
                end = r2;
            }
        }

        letters[start as usize..=end as usize].iter().collect()
    }
}
