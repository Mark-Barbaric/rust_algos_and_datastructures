pub fn bs(_target: i32, _nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = _nums.len() - 1;

    while l < r {
        let m = l + ((r - l) / 2);

        if _nums[m] == _target {
            return m as i32;
        }

        if _target > _nums[m] {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    return -1;
}

#[cfg(test)]
mod binary_search1_tests {
    use super::*;

    #[test]
    fn returns_idx() {
        let nums = vec![1, 2, 3, 4];
        let ans = bs(2, nums);
        assert_eq!(ans, 1);
    }

    #[test]
    fn not_found() {
        let nums = vec![1, 2, 3, 4];
        let ans = bs(22, nums);
        assert_eq!(ans, -1);
    }
}