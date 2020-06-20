pub fn brackets_are_balanced(string: &str) -> bool {
    if string.is_empty() { return true }

    let mut filtered_str = string.to_string();
    // Filtering characters other than parentheses
    filtered_str.retain(|c| String::from("{}()[]").contains(c) );

    // The size must be even so that the brackets are a pair
    if filtered_str.len() % 2 != 0 { return false }

    // Filtering each pair.
    let filtered_str = filtered_str.replace("[]", "");
    let filtered_str = filtered_str.replace("{}", "");
    let filtered_str = filtered_str.replace("()", "");

    let limit = filtered_str.len() / 2;
    let (left, right) = filtered_str.split_at(limit);

    let mut right_iter = right.chars().rev();

    for left_char in left.chars().into_iter() {
        let right_char = right_iter.next();
        let valid = match left_char {
            '{' if right_char == Some('}') => true,
            '[' if right_char == Some(']') => true,
            '(' if right_char == Some(')') => true,
            _ => false
        };

        if !valid { return false }
    }
    true
}
