//20. Valid Parentheses
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' => {
                    if Some(ch) != stack.pop() {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        stack.is_empty()
    }
}
