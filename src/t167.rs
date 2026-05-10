#[allow(dead_code)]
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left < right {
        let s = numbers[left] + numbers[right];

        if s == target {
            return vec![left as i32 + 1, right as i32 + 1];
        } else if s > target {
            right -= 1;
        } else {
            left += 1;
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_167_1() {
        // Source: 167-两数之和 II.md
        // 示例 1：
        // 输入：numbers = [2,7,11,15], target = 9
        // 输出：[1,2]
        assert_eq!(vec![1, 2], two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn test_167_2() {
        // Source: 167-两数之和 II.md
        // 示例 2：
        // 输入：numbers = [2,3,4], target = 6
        // 输出：[1,3]
        assert_eq!(vec![1, 3], two_sum(vec![2, 3, 4], 6));
    }

    #[test]
    fn test_167_3() {
        // Source: 167-两数之和 II.md
        // 示例 3：
        // 输入：numbers = [-1,0], target = -1
        // 输出：[1,2]
        assert_eq!(vec![1, 2], two_sum(vec![-1, 0], -1));
    }
}
