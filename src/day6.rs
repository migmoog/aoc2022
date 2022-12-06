pub fn day_6(input: &str) {
    let chars = input.chars().collect::<Vec<_>>();
    // part 1
    for i in 0..chars.len()-4 {
        if is_marker(&chars[i..i+4]) {
            println!("Day 6 (part 1) {}", i+4);
            break;
        }
    }

    // part 2
    for i in 0..chars.len()-14 {
        if is_marker(&chars[i..i+14]) {
            println!("Day 6 (part 2) {}", i+14);
            break;
        }
    }
}

fn is_marker(sl: &[char]) -> bool {
    for i in 0..sl.len() {
        let current = sl[i];
        if sl[i+1..sl.len()].iter().any(|&c| current == c) {
            return false;
        }
    }

    true
}