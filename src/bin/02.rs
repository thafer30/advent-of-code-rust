advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_reports = 0;
    for line in input.lines() {
        let iter = line.split_ascii_whitespace();

        let levels: Vec<u32> = iter
            .map(|item| item.parse::<u32>().unwrap())
            .collect();

        let is_safe = are_levels_safe(&levels);

        if is_safe {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
}

fn are_levels_safe(levels: &[u32]) -> bool {
    let mut is_safe = true;
    let mut is_increasing = None;
    for (index, item) in levels.into_iter().enumerate() {
        if index > 0 {
            let difference = *item as i32 - levels[index - 1] as i32;

            let tmp_is_increasing = Some(difference > 0);

            if is_increasing == None {
                is_increasing = tmp_is_increasing;
            } else {
                if is_increasing != tmp_is_increasing {
                    is_safe = false;
                    break;
                }
            }

            let difference = item.abs_diff(levels[index - 1]);

            if difference > 3 || difference < 1 {
                is_safe = false;
                break;
            }
        }
    }

    is_safe
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_reports = 0;
    for line in input.lines() {
        let iter = line.split_ascii_whitespace();

        let levels: Vec<u32> = iter
            .map(|item| item.parse::<u32>().unwrap())
            .collect();

        let is_safe = are_levels_safe(&levels);

        if is_safe {
            safe_reports += 1;
        } else {
            for (index, _) in levels.clone().into_iter().enumerate() {
                let mut tmp_levels = levels.clone();
                tmp_levels.remove(index);
                if are_levels_safe(&tmp_levels) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }

    Some(safe_reports)
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
