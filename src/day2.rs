use Move::*;
use std::cmp::Ordering::{self, *};

#[derive(PartialEq, Clone)]
enum Move {
    Rock, Paper, Scissor
}
impl Move {
    fn weakness(self) -> Self {
        match self {
            Rock => Paper,
            Paper => Scissor,
            Scissor => Rock
        }
    }

    fn to_i32(self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        }
    }
}
impl TryFrom<char> for Move {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Rock),
            'B' | 'Y' => Ok(Paper),
            'C' | 'Z' => Ok(Scissor),
            _ => Err(
                "Character is not one of the beginning or ending 3 letters of the alphabet"
            )
        }
    }
}
impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // equal is equivalent to a draw
        if self == other {return Some(Equal)}

        match self {
            &Rock => {
                if other == &Scissor {
                    Some(Greater)
                } else {
                    Some(Less)
                }
            },
            &Paper => {
                if other == &Rock {
                    Some(Greater)
                } else {
                    Some(Less)
                }
            },
            &Scissor => {
                if other == &Paper {
                    Some(Greater)
                } else {
                    Some(Less)
                }
            }
        }
    }
}

pub fn day_2(input: &str) {
    let mut score = 0;
    let mut part2_score = 0;

    for l in input.lines() {
        let mut chars = l.chars();

        let opponent = match chars.find(|c| {
            let code = *c as u8;
            65 <= code && code <= 67
        }) {
            Some(c) => Move::try_from(c).unwrap(),
            None => unreachable!()
        };

        let expected_result;
        let player = match chars.find(|c| {
            let code = *c as u8;
            88 <= code && code <= 90
        }) {
            Some(c) => {
                expected_result = c;
                Move::try_from(c).unwrap()
            },
            None => unreachable!()
        };

        let outcome = if player > opponent {
            6
        } else if player == opponent {
            3
        }  else {
            0
        };

        let move_bonus = player.to_i32();
        score += outcome + move_bonus;

        // part 2
        let part2_player = opponent.clone();
        match expected_result {
            'X' => {
                // must lose, so add the players weakness
                part2_score += part2_player.weakness().to_i32();
            }
            'Y' => {
                part2_score += part2_player.to_i32() + 3;
            }
            'Z' => {
                part2_score += part2_player.to_i32() + 6;
            }
            _ => {}
        }
    }

    println!("Day 2 (part 1) {score}");
    println!("Day 2 (part 2) {part2_score}");
}