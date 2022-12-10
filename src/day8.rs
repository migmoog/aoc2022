pub fn day_8(input: &str) {
    let mut grid = {
        let mut out = Vec::<Vec<i32>>::new();
        for l in input.lines() {
            out.push(l.chars().map(
                |c| c.to_digit(10).unwrap() as i32,
            ).collect());
        }
        out
    };

    let mut result = ((2 * (grid.len() + grid[0].len())) - 4) as i32;
    let mut scenic_scores = Vec::new();
    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            let h = grid[y][x];
            let mut left = true;
            let mut leftward = 0;
            for i in (0..x).rev() {
                leftward += 1;
                if grid[y][i] >= h {
                    left = false;
                    break;
                }
            }

            let mut right = true;
            let mut rightward = 0;
            for i in x + 1..grid[y].len() {
                rightward += 1;
                if grid[y][i] >= h {
                    right = false;
                    break;
                }
            }

            let mut up = true;
            let mut upward = 0;
            for i in (0..y).rev() {
                upward += 1;
                if grid[i][x] >= h {
                    up = false;
                    break;
                }
            }

            let mut down = true;
            let mut downward = 0;
            for i in y + 1..grid.len() {
                downward += 1;
                if grid[i][x] >= h {
                    down = false;
                    break;
                }
            }

            if left || right || up || down {
                result += 1;
            }

            scenic_scores.push(leftward * rightward * upward * downward);
        }
    }

    let mut best_scenic_score = 0;
    for i in scenic_scores {
        if i > best_scenic_score {
            best_scenic_score = i;
        }
    }

    println!("Day 8 (part 1) {result}");
    println!("Day 8 (part 2) {best_scenic_score}");
}
