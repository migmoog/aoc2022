use std::ops::RangeInclusive;

pub fn day_4(input: &str) {
    let mut result = 0;
    let mut part2_result = 0;

    for l in input.lines() {
        let mut spl = l.split(',');
        let first_elf_str = spl.next().unwrap();
        let second_elf_str = spl.next().unwrap();

        let first_elf = range_from_str(first_elf_str);
        let second_elf = range_from_str(second_elf_str);

        // part 1
        let contains = (
            first_elf.clone().all(|i| second_elf.clone().contains(&i)),
            second_elf.clone().all(|i| first_elf.clone().contains(&i))
        );

        if contains.0 || contains.1 {
            result += 1;
        }

        // part 2
        let overlaps = (
            first_elf.clone().any(|i| second_elf.clone().contains(&i)),
            second_elf.clone().any(|i| first_elf.clone().contains(&i))
        );

        if overlaps.0 || overlaps.1 {
            part2_result += 1;
        }
    }

    println!("Day 4 (part 1) {}", result);
    println!("Day 4 (part 2) {}", part2_result);
}

fn range_from_str(s: &str) -> RangeInclusive<i32> {
    let mut spl = s.split('-');
    let left_dig = spl.next().unwrap();
    let right_dig = spl.next().unwrap();
    left_dig.parse::<i32>().unwrap()..=right_dig.parse::<i32>().unwrap()
}