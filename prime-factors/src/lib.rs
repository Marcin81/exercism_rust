pub fn factors(n: u64) -> Vec<u64> {
    let mut divisions = vec![];
    let mut number = n;
    let mut divisor = 2u64;

    while divisor <= number {
        if number % divisor == 0 {
            divisions.push(divisor);
            number = number / divisor;
        } else {
            divisor += 1;
        }
    }
    divisions
}
