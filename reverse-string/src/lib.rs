use std::str;
use std::fmt::Debug;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
//    return input.chars().rev().collect();
    return input.graphemes(true).rev().collect();
}
