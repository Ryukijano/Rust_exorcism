pub fn verse(n: u32) -> String {
    let mut result = String::new();
    if n == 0 {
        result.push_str("No more bottles of beer on the wall, no more bottles of beer.\n");
        result.push_str("Go to the store and buy some more, 99 bottles of beer on the wall.\n");
    } else if n == 1 {
        result.push_str("1 bottle of beer on the wall, 1 bottle of beer.\n");
        result.push_str("Take it down and pass it around, no more bottles of beer on the wall.\n");
    } else {
        result.push_str(&format!("{} bottles of beer on the wall, {} bottles of beer.\n", n, n));
        result.push_str(&format!("Take one down and pass it around, {} bottles of beer on the wall.\n", n - 1));
    }
    result
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();
    for n in (end..start + 1).rev() {
        result.push_str(&verse(n));
        result.push_str("\n");
    }
    result
}
