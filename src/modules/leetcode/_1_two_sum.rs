use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (index, &num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(target_index) = map.get(&diff) {
            return vec![*target_index as i32, index as i32];
        }
        map.insert(num, index as i32);
    }
    vec![]
}

#[test]
fn test() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}