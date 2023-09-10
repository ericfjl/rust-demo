fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for &num in numbers {
        // 使用 checked_add 方法进行安全的加法运算，以防止整数溢出
        match sum.checked_add(num) {
            Some(new_sum) => sum = new_sum,
            None => return None, // 如果出现溢出，返回 None
        }
    }
    Some(sum) // 如果没有溢出，返回 Some(sum)
}
pub fn test(){
    let numbers1 = [10, 20, 30, 40];
    let numbers2 = [u32::MAX, 1];

    match sum_u32(&numbers1) {
        Some(sum) => println!("The sum of numbers1 is: {}", sum),
        None => println!("Overflow occurred for numbers1"),
    }

    match sum_u32(&numbers2) {
        Some(sum) => println!("The sum of numbers2 is: {}", sum),
        None => println!("Overflow occurred for numbers2"),
    }
}





