//4. Median of Two Sorted Arrays
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged = nums1;
        merged.extend(nums2);
        merged.sort_unstable(); // Daha hızlı, sıralama garantisi gerekmediğinde

        let len = merged.len();
        let mid = len / 2;

        if len % 2 == 0 {
            (merged[mid - 1] + merged[mid]) as f64 / 2.0
        } else {
            merged[mid] as f64
        }
    }
}
