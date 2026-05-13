use std::collections::VecDeque;

#[allow(dead_code)]
fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut ans = Vec::with_capacity(nums.len() - k + 1);

    let mut queue = VecDeque::new();

    for (i, &x) in nums.iter().enumerate() {
        // 当队列不为空，并且最后的数字比当前值大时，将之前的值丢弃，保留当前值
        // 队列为从大到小的降序
        while !queue.is_empty() && nums[*queue.back().unwrap()] <= x {
            queue.pop_back();
        }
        // 将当前的下标放到队列中
        queue.push_back(i);
        // 当前下标为i，窗口区间为 [i-k+1, i]
        // 判断队列中第一个是否已经不在窗口范围内，如果不在则推出
        if queue[0] <= i - k {
            queue.pop_front();
        }
        // 只有当 i+1 >=k 时，才产生窗口
        // 将队列中第一个值放到结果中（存的是下标）
        if i + 1 >= k {
            ans.push(nums[queue[0]]);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_239_1() {
        // Source: 239-滑动窗口最大值.md
        // 示例 1：
        // 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
        // 输出：[3,3,5,5,6,7]
        assert_eq!(
            vec![3, 3, 5, 5, 6, 7],
            max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
        );
    }

    #[test]
    fn test_239_2() {
        // Source: 239-滑动窗口最大值.md
        // 示例 2：
        // 输入：nums = [1], k = 1
        // 输出：[1]
        assert_eq!(vec![1], max_sliding_window(vec![1], 1));
    }
}
