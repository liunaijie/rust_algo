
#[allow(dead_code)]
pub fn move_zeroes(nums: &mut Vec<i32>) {
    // 表示从左侧0开始的位置，用于将右侧非0数字放到这个位置
    let mut zero_start = 0;
    for index in 0..nums.len() {
        // 如果值不为0，则将值与zero_start位置的元素交换
        if nums[index] != 0 {
            nums.swap(zero_start, index);
            // 左侧0开始的位置向后移动一位
            zero_start += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests_move_zeroes_1() {
        let mut nums = vec![0,1,0,3,12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1,3,12,0,0]);
    }

    #[test]
    fn tests_move_zeroes_2() {
        let mut nums = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}