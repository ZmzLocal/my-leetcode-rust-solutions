//67. Add Binary

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::new();
        let mut carry = 0;
        
        let mut a_iter = a.chars().rev();
        let mut b_iter = b.chars().rev();
        
        while true {
            let a_bit = a_iter.next();
            let b_bit = b_iter.next();
            
            if a_bit.is_none() && b_bit.is_none() && carry == 0 {
                break;
            }
            
            let mut sum = carry;
            
            if let Some(c) = a_bit {
                if c == '1' {
                    sum += 1;
                }
            }
            
            if let Some(c) = b_bit {
                if c == '1' {
                    sum += 1;
                }
            }
            
            match sum {
                0 => {
                    result.push('0');
                    carry = 0;
                },
                1 => {
                    result.push('1');
                    carry = 0;
                },
                2 => {
                    result.push('0');
                    carry = 1;
                },
                3 => {
                    result.push('1');
                    carry = 1;
                },
                _ => unreachable!(),
            }
        }
        
        
        result.chars().rev().collect()
    }
}
