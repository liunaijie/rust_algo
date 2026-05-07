use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // 使用HashMap记录已经遍历过的元素和其索引，避免重复计算
    let mut map = HashMap::new();
    // 结果中要求返回下标，所以需要记录下标
    for (index, num) in nums.iter().enumerate() {
        let complement = target - num;
        // 如果complement在map中，则返回complement的index和当前index
        if map.contains_key(&complement) {
            return vec![map[&complement], index as i32];
        }
        // 如果没找到，则将当前元素和index插入到map中
        map.insert(num, index as i32);
        // 由于不能使用同一个元素两次，所以这里是先查找后插入，避免重复使用同一个元素
    }
    // 如果没找到，则返回空数组
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum1() {
        let nums = vec![2,7,11,15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0,1]);
    }
    #[test]
    fn test_two_sum2() {
        let nums = vec![3,2,4];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![1,2]);
    }
    #[test]
    fn test_two_sum3() {
        let nums = vec![3,3];
        let result = two_sum(nums,6);
        assert_eq!(result, vec![0,1]);
    }
}