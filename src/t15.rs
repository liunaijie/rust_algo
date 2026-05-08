#[allow(dead_code)]
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut ans = vec![];
    let n = nums.len();
    for i in 0..n - 2 {
        let x = nums[i];
        // 跳过相同的元素，
        if i > 0 && x == nums[i - 1] {
            continue;
        }

        // 由于数组有序，当i位置的最小三个值相加都大于0时，表明不会存在三个值相加等于0
        if x + nums[i + 1] + nums[i + 2] > 0 {
            break;
        }
        // i位置的最大值如果比0小，则i位置不会存在结果
        // 但是i+1位置可能存在，所以仅需跳过此次，不能跳出循环
        if x + nums[n - 2] + nums[n - 1] < 0 {
            continue;
        }

        let mut j = i + 1;
        let mut k = n - 1;

        while j < k {
            let s = x + nums[j] + nums[k];
            if s < 0 {
                j += 1;
            } else if s > 0 {
                k -= 1;
            } else {
                // 找到一组答案，将答案添加到结果集中
                ans.push(vec![x, nums[j], nums[k]]);
                // 左右指针收缩，但是由于可能存在重复元素，所以还需要额外跳过重复数字
                j += 1;
                while j < k && nums[j] == nums[j - 1] {
                    // 跳过重复数字
                    j += 1;
                }
                k -= 1;
                while k > j && nums[k] == nums[k + 1] {
                    // 跳过重复数字
                    k -= 1;
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15_1() {
        // Source: 15-三数之和.md
        // 示例 1：
        // 输入：nums = [-1,0,1,2,-1,-4]
        // 输出：[[-1,-1,2],[-1,0,1]]
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
    }

    #[test]
    fn test_15_2() {
        // Source: 15-三数之和.md
        // 示例 2：
        // 输入：nums = [0,1,1]
        // 输出：[]
        assert_eq!(Vec::<Vec<i32>>::new(), three_sum(vec![0, 1, 1]));
    }

    #[test]
    fn test_15_3() {
        // Source: 15-三数之和.md
        // 示例 3：
        // 输入：nums = [0,0,0]
        // 输出：[[0,0,0]]
        assert_eq!(vec![vec![0, 0, 0]], three_sum(vec![0, 0, 0]));
    }
}
