static BOTTLES: &str = "bottles of beer";
static BOTTLE: &str = "bottle of beer";
static WALL: &str = "on the wall";
static TAKE_IT: &str = "Take it down and pass it around";
static TAKE_ONE: &str = "Take one down and pass it around";
static NO_MORE: &str = "no more";
static NO_MORE_UPPER: &str = "No more";
static STORE: &str = "Go to the store and buy some more";

pub fn verse(n: u32) -> String {
    match n {
        2 => format!("{} {} {}, {} {}.\n{}, 1 {} {}.\n", n, BOTTLES, WALL, n, BOTTLES, TAKE_ONE, BOTTLE, WALL),
        1 => format!("{} {} {}, {} {}.\n{}, {} {} {}.\n", n, BOTTLE, WALL, n, BOTTLE, TAKE_IT, NO_MORE, BOTTLES, WALL),
        0 => format!("{} {} {}, {} {}.\n{}, 99 {} {}.\n", NO_MORE_UPPER, BOTTLES, WALL, NO_MORE, BOTTLES, STORE, BOTTLES, WALL),
        _ => format!("{} {} {}, {} {}.\n{}, {} {} {}.\n", n, BOTTLES, WALL, n, BOTTLES, TAKE_ONE, n - 1, BOTTLES, WALL)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(verse).collect::<Vec<String>>().join("\n")
}
