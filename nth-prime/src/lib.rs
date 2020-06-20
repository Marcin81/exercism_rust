pub fn nth(n: u32) -> u32 {
    let mut prime = 2;
    let mut index = 0;

    loop {
        if is_prime(prime) {
            if index == n {
                break;
            }
            index += 1;
        }
        prime += 1;
    }
    prime
}

fn is_prime(num: u32) -> bool {
    let mut n = 2;
    while n < num {
        if num % n == 0 { return false }
        n += 1;
    }
    true
}
