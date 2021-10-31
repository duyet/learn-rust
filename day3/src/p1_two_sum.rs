pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for x in 0..nums.len() {
            for y in (x + 1)..nums.len() {
                if nums[x] + nums[y] == target {
                    return vec![x as i32, y as i32];
                }
            }
        }
        vec![]
    }
    
    pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            println!("i = {}, num = {}, complement = {}, map = {:?}", i, num, complement, map);
            if map.contains_key(&complement) {
                return vec![map[&complement] as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}

#[test]
fn test_1() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 3], Solution::two_sum_hash(vec![1, 2, 5, 7, 11, 15], 9));
}
