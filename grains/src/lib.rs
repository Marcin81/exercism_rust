
pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 { panic!("Square must be between 1 and 64") }
    if s == 1 { return 1 }
    square(s-1) *2
}

pub fn total() -> u64 {
    let mut sum= square(64);
    let mut result = 0;
    for _n in 1..65 {
        result += sum;
        sum /= 2;
    }
    result
}
