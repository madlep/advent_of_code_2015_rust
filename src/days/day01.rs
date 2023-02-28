type Floor = i32;
type Instruction = char;

pub fn part1(data: &str) -> String {
    data.chars().fold(0, handle_instruction).to_string()
}

pub fn part2(data: &str) -> String {
    data.chars()
        .enumerate()
        .try_fold(0, |current_floor, (i, instruction)| {
            let new_floor = handle_instruction(current_floor, instruction);
            if new_floor < 0 {
                Err(i + 1)
            } else {
                Ok(new_floor)
            }
        })
        .unwrap_err()
        .to_string()
}

fn handle_instruction(current_floor: Floor, instruction: Instruction) -> Floor {
    match instruction {
        '(' => current_floor + 1,
        ')' => current_floor - 1,
        c => panic!("unexpected character {}", c),
    }
}
