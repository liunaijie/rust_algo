#[allow(dead_code)]
pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_unstable_by_key(|p| p[0]);
    let mut ans: Vec<Vec<i32>> = vec![];
    for p in intervals {
        if let Some(last) = ans.last_mut() {
            if p[0] <= last[1] {
                last[1] = last[1].max(p[1]);
            } else {
                ans.push(p);
            }
        } else {
            ans.push(p);
        };
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_56_1() {
        // Source: 56-合并区间.md
        // 示例 1：
        // 输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
        // 输出：[[1,6],[8,10],[15,18]]
        assert_eq!(
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
            merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
        );
    }

    #[test]
    fn test_56_2() {
        // Source: 56-合并区间.md
        // 示例 2：
        // 输入：intervals = [[1,4],[4,5]]
        // 输出：[[1,5]]
        assert_eq!(vec![vec![1, 5]], merge(vec![vec![1, 4], vec![4, 5]]));
    }

    #[test]
    fn test_56_3() {
        // Source: 56-合并区间.md
        // 示例 3：
        // 输入：intervals = [[4,7],[1,4]]
        // 输出：[[1,7]]
        assert_eq!(vec![vec![1, 7]], merge(vec![vec![4, 7], vec![1, 4]]));
    }
}
