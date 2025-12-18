//11. Container With Most Water
fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;

        while left < right {
            let h = min(height[left], height[right]);
            let w = (right - left) as i32;
            let area = h * w;
            if max_area < area{
                max_area = area
            }

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}

