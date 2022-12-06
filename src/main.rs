mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use std::env::consts::OS;
use day2::day_2;
use crate::day3::day_3;
use day4::day_4;
use crate::day5::day_5;
use crate::day6::day_6;

fn main() {
    day_1(include_str!("day1.txt"));
    day_2(include_str!("day2.txt"));
    day_3(include_str!("day3.txt"));
    day_4(include_str!("day4.txt"));
    day_5(include_str!("day5.txt"));
    day_6(include_str!("day6.txt"));
}

fn day_1(input: &str) {
    // part 1
    let group_strs = input.split(
        if OS == "windows" {
            // windows whyyyyy
            "\r\n\r\n"
        } else { "\n\n" });

    let mut groups = Vec::new();
    for g in group_strs {
        let total_elf_cals: i32 = g.lines().map(
            |s| s.parse::<i32>().unwrap()
        ).sum();
        groups.push(total_elf_cals);
    }

    groups.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("Day 1 (part 1): {}", groups[0]);
    println!("Day 1 (part 2): {}", groups[0..3].iter().sum::<i32>());
}