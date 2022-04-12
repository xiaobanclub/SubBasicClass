fn sum(nums: &[u32]) -> Option<u32> {
    let mut total: u64 = 0;
    for num in nums.iter() {
        total = total + *num as u64;
        if total > std::u32::MAX as u64 {
            return None;
        }
    }
    return Some(total as u32);
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    let total = sum(&a);
    println!("{}", total.unwrap()); // 15

    let a = [1, std::u32::MAX];
    let total = sum(&a);
    println!("{}", total.is_none()) // true
}
