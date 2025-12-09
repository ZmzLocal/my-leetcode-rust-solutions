//6. Zigzag Conversion

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || s.len() <= num_rows as usize {
            return s;
        }

        let mut rows: Vec<String> = vec![String::new(); num_rows as usize]; // row kadar string içersin
        let mut current_row = 0;
        let mut going_down = false;

        for c in s.chars() {
            rows[current_row].push(c);
            if current_row == 0 || current_row == (num_rows - 1) as usize {
                going_down = !going_down;
            }
            if going_down {
                current_row += 1
            } else {
                current_row -= 1
            };
        }

        let mut result : String = "".to_string();
        for row in &rows {
        result.push_str(row);
        }
        result
    }
}
