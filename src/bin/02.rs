advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_reports = 0;

    for line in input.lines() {
        let levels: Vec<u32> = line
            .split_whitespace()
            .map(|item| item.parse::<u32>().unwrap())
            .collect();

        if are_levels_safe(&levels) {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
}

fn are_levels_safe(levels: &[u32]) -> bool {
    let mut is_increasing = None;

    for window in levels.windows(2) {
        let diff = window[1] as i32 - window[0] as i32;

        if let Some(prev_increasing) = is_increasing {
            if prev_increasing != (diff > 0) {
                return false;
            }
        } else {
            is_increasing = Some(diff > 0);
        }

        let diff_abs = diff.abs();

        if diff_abs > 3 || diff_abs < 1 {
            return false;
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_reports = 0;

    for line in input.lines() {
        let levels: Vec<u32> = line
            .split_whitespace()
            .map(|item| item.parse::<u32>().unwrap())
            .collect();

        // First, check if the current levels are safe
        if are_levels_safe(&levels) {
            safe_reports += 1;
        } else {
            // Only check for removals if the sequence is unsafe
            if check_level_removals(&levels) {
                safe_reports += 1;
            }
        }
    }

    Some(safe_reports)
}

fn check_level_removals(levels: &[u32]) -> bool {
    for index in 0..levels.len() {
        // Try removing the level at index `i`
        let mut tmp_levels = levels.to_vec(); // Create a copy of the levels
        tmp_levels.remove(index);

        // Check if the modified levels are safe
        if are_levels_safe(&tmp_levels) {
            return true; // Found a safe configuration after removal
        }
    }

    false // No valid removal made the sequence safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
