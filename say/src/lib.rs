/*
1. Handle the basic case of 0 through 99.

2. Implement breaking a number up into chunks of thousands.

So 1234567890 should yield a list like 1, 234, 567, and 890, while the far simpler 1000 should yield just 1 and 0.

3.Now handle inserting the appropriate scale word between those chunks.

So 1234567890 should yield '1 billion 234 million 567 thousand 890'

4. Put it all together to get nothing but plain English.

12345 should give twelve thousand three hundred forty-five.
 */
pub fn encode(n: u64) -> String {
    let power_num: u32 = (f32::log10(n as f32)) as u32;
    // println!("n: {}", n);
    // println!("power_num: {}", power_num);
    let last_digit = n%10;
    // let first_digit = n / 10u32.pow(power_num);
    let first_digit = n / 10u64.pow(power_num);
    let shorter_number = n % 10u64.pow(power_num);
    // println!("first_digit: {}", first_digit);
    // println!("shorter_number: {}", shorter_number);
    // println!("last_digit: {}", last_digit);
    if n == 0 { return "zero".to_string() }
    match n {
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13..=19 => format!("{}teen", encode(last_digit)),
        20 => String::from("twenty"),
        21..=29 => format!("twenty-{}", encode(last_digit)),
        30 => String::from("thirty"),
        31..=39 => format!("thirty-{}", encode(last_digit)),
        40 => String::from("forty"),
        41..=49 => format!("forty-{}", encode(last_digit)),
        50 => String::from("fifty"),
        51..=59 => format!("fifty-{}", encode(last_digit)),
        60 => String::from("sixty"),
        61..=69 => format!("sixty-{}", encode(last_digit)),
        70 => String::from("seventy"),
        71..=79 => format!("seventy-{}", encode(last_digit)),
        80 => String::from("eighty"),
        81..=89 => format!("eighty-{}", encode(last_digit)),
        90 => String::from("ninety"),
        91..=99 => format!("ninety-{}", encode(last_digit)),
        100 => String::from("one hundred"),
        101..=999 => format!("{} hundred {}", encode(first_digit), encode(shorter_number)),
        1000 => String::from("one thousand"),
        // nieprawidlowe zakresy od tego miejsca popraw
        1001..=9999 => format!("{} thousand {}", encode(first_digit), encode(shorter_number)),
        1_000_000 => String::from("one million"),
        1_000_001..=999_999_999 => format!("{} million {}", encode(first_digit), encode(shorter_number)),
        1_000_000_000 => String::from("one billion"),
        1_000_000_001..=9_999_999_999 => format!("{} billion {}", encode(first_digit), encode(shorter_number)),
        // 10_000_000_000..=19_000_000_000 =>
        // 1_446_744_073_709_551_615,
        _ => String::from("dupa")
    }
    // String::from("zero")10000000000
}
