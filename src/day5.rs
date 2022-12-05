pub fn day_5(input: &str) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    /*move (0) from (1) to (2)*/
    let mut instructions: Vec<[usize; 3]> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();

    let mut stacks_lines:Vec<&str> = Vec::new();

    // get lines of brackets
    let mut labels_idx = 0;
    let mut labels_positions: Vec<usize> = Vec::new();
    for i in 0..lines.len() {
        if !lines[i].contains('[') {
            labels_idx = i;
            for (j, c) in lines[i].chars().enumerate() {
                if !c.is_ascii_digit() {continue;}

                labels_positions.push(j);
            }

            for j in 0..labels_idx {
                stacks_lines.push(lines[j]);
            }
            lines[labels_idx].chars().for_each(|c| {
                if c.is_ascii_digit() {
                    stacks.push(Vec::new());
                }
            });
            break;
        }
    }

    for sl in stacks_lines {
        for (i, c) in sl.chars().enumerate() {
            if !c.is_alphabetic() { continue; }

            stacks[{
                let mut stack_idx = 0;
                for j in 0..labels_positions.len() {
                    if labels_positions[j] == i {
                        stack_idx = j;
                        break;
                    }
                }
                stack_idx
            }].push(c);
        }
    }
    // crate "parsing"
    for stk in stacks.iter_mut() {
        stk.reverse();
    }

    // instruction "parsing"
    for i in labels_idx+2..lines.len() {
        let mut instr = [0usize, 0, 0];
        let mut instr_index = 0;
        for res in lines[i].split_whitespace().map(|s| s.parse::<usize>()) {
            let Ok(int) = res else {
                continue;
            };

            instr[instr_index] = int;
            instr_index += 1;
        }
        instr[1] -= 1;
        instr[2] -= 1;

        instructions.push(instr);
    }

    let mut stacks_part2 = stacks.clone();
    for instr in instructions {
        let [amount, from, to] = instr;
        let mut moved = Vec::new();


        let mut moved_part2 = Vec::new();
        for _ in 0..amount {
            moved.push(stacks[from].pop().unwrap());
            moved_part2.push(stacks_part2[from].pop().unwrap());
        }
        moved_part2.reverse();

        for c in moved { stacks[to].push(c); }
        for c in moved_part2 { stacks_part2[to].push(c); }
    }

    let mut tops_message = String::new();
    for stk in stacks {
        tops_message.push(stk[stk.len()-1]);
    }
    let mut tops_message_part2 = String::new();
    for stk in stacks_part2 {
        tops_message_part2.push(stk[stk.len()-1]);
    }

    println!("Day 5 (part 1) {}", tops_message);
    println!("Day 5 (part 2) {}", tops_message_part2);
}