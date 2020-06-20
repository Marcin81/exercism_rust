pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();
    if list.is_empty() { return result }

    let mut copy_list: Vec<_> = list.iter().collect();
    for _i in 1..copy_list.len() {
        copy_list.rotate_left(1);
        result = format!("{}{}\n", result, make_sentence(copy_list.last().unwrap(), copy_list.first().unwrap()));
    }
    result = format!("{}And all for the want of a {}.", result, list.first().unwrap());
    result
}

fn make_sentence(word1: &str, word2: &str) -> String {
    format!("For want of a {} the {} was lost.", word1, word2)
}
