#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut index = 0;
    while index < nums.len() {
        for i in 0..nums.len() as i32 {
            // duplicate indices are not allowed
            if index == i as usize { continue };

            if nums[index] + nums[i as usize] == target {
                return vec![index as i32, i];
            }
        }
        index += 1;
    }
    // dummy
    vec![0,0]
}