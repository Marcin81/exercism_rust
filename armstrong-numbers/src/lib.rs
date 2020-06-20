pub fn is_armstrong_number(num: u32) -> bool {
    let power_num: u32 = (f32::log10(num as f32) + 1.0) as u32;
    // fast
    let mut number = num;
    let mut sum = 0;
    while number > 0 {
        let digit = number%10;
        number /= 10;
        sum += u32::pow(digit, power_num);
    }
    sum == num
    //slow:
//    (1..num+1).scan(num, |number, _| {
//        let digit = *number%10;
//        *number /= 10;
//        Some(u32::pow(digit, power_num))
//    }).sum::<u32>() == num
}
