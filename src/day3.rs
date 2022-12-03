pub fn day_3(input: &str) {
    let mut result = 0;
    let lines: Vec<&str> = input.lines().collect();

    // part 1
    for l in lines.iter() {
        let (
            left_compartment,
            right_compartment
        ) = l.split_at(l.len() / 2);


        for cl in left_compartment.chars() {
            match right_compartment.chars().find(|&c| c == cl) {
                Some(c) => {
                    result += priority(c);
                    break;
                }
                None => {}
            }
        }
    }

    // part 2
    let mut part2_result = 0;
    for i in (0..lines.len()-2).step_by(3) {
        let (first, second, third) = (lines[i], lines[i+1], lines[i+2]);

        for c1 in first.chars() {
            if let Some(c2) = second.chars().find(|&c| c == c1) {
                if let Some(c3) = third.chars().find(|&c| c == c2) {
                    part2_result += priority(c3);                    
                    break;
                }
            }
        }
    }

    println!("Day 3 (part 1) {}", result);
    println!("Day 3 (part 2) {}", part2_result);
}

fn priority(c: char) -> i32 {
    let mut alphabet_index = c.to_ascii_lowercase() as i32 - 96;
    if c.is_ascii_uppercase() {
        alphabet_index += 26;
    }

    alphabet_index
}