#[allow(dead_code)]
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut left_index = 0;
    let mut right_index = height.len() - 1;

    while left_index < right_index {
        let area = height[left_index].min(height[right_index]) * (right_index - left_index) as i32;
        max_area = max_area.max(area);
        if height[left_index] < height[right_index] {
            left_index += 1;
        } else {
            right_index -= 1;
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11_1() {
        assert_eq!(49, max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }

    #[test]
    fn test_11_2() {
        assert_eq!(1, max_area(vec![1, 1]));
    }
}
