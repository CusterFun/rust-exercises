fn ordered_sequential_search(nums: &[i32], num: i32) -> bool {
    let mut pos = 0;
    let mut found = false;
    let mut stop = false; // 控制遇到有序数据时退出

    while pos < nums.len() && !found && !stop {
        if num == nums[pos] {
            found = true;
        } else if num < nums[pos] {
            stop = true; // 数据有序，退出
        } else {
            pos += 1;
        }
    }
    found
}

fn main() {
    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];
    let num = 44;
    let found = ordered_sequential_search(&nums, num);
    println!("{num} is in nums: {found}");

    let num = 49;
    let found = ordered_sequential_search(&nums, num);
    println!("{num} is in nums: {found}");
}
