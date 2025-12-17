//8. String to Integer (atoi)

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start();
        let mut chars = s.chars().peekable();
        let mut is_negative = false;

        if let Some(&c) = chars.peek() {
            if c == '-' {
                is_negative = true;
                chars.next();
            } else if c == '+' {
                chars.next();
            }
        }

        let mut number_str = String::new();
        for c in chars {
            if c.is_ascii_digit() {
                number_str.push(c);
            } else {
                break;
            }
        }

        if number_str.is_empty() {
            return 0;
        }

        let mut number = match number_str.parse::<i64>() {
            Ok(n) => {
                if is_negative {
                    -n
                } else {
                    n
                }
            }
            Err(_) => {
                return if is_negative { i32::MIN } else { i32::MAX };
            }
        };

        if number < i32::MIN as i64 {
            i32::MIN
        } else if number > i32::MAX as i64 {
            i32::MAX
        } else {
            number as i32
        }
    }
}
