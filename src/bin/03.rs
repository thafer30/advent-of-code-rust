advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;

    // Loop through each line
    for line in input.lines() {
        let parts = line.split("mul(");

        for (index, item) in parts.enumerate() {
            // Skip the first item
            if index == 0 {
                continue;
            }

            // Extract the numbers after 'mul(' and ','
            if let Some((first_str, rest)) = item.split_once(',') {
                if let Ok(first_val) = first_str.parse::<u32>() {
                    if let Some((second_str, _)) = rest.split_once(')') {
                        if let Ok(second_val) = second_str.parse::<u32>() {
                            total += first_val * second_val;
                        }
                    }
                }
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let mut enabled = true;
    let mut next_enabled = enabled;

    // Loop through each line
    for line in input.lines() {
        let parts = line.split("mul(");

        for (index, item) in parts.enumerate() {
            enabled = next_enabled;

            let do_index = item.rfind("do()");
            let dont_index = item.rfind("don't()");

            // Determine the next_enabled state
            next_enabled = match (do_index, dont_index) {
                (Some(_), None) => true,  // Only "do" found
                (None, Some(_)) => false, // Only "don't" found
                (Some(do_pos), Some(dont_pos)) => {
                    if do_pos > dont_pos {
                        true
                    } else {
                        false
                    } // Decide based on the positions
                }
                _ => next_enabled, // No change if neither are found
            };

            // Skip the first item
            if index == 0 {
                continue;
            }

            // Extract the numbers after 'mul(' and ','
            if let Some((first_str, rest)) = item.split_once(',') {
                if let Ok(first_val) = first_str.parse::<u32>() {
                    if let Some((second_str, _)) = rest.split_once(')') {
                        if let Ok(second_val) = second_str.parse::<u32>() {
                            if enabled {
                                total += first_val * second_val;
                            }
                        }
                    }
                }
            }
        }
    }

    Some(total)
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
