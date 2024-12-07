advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;

    // Loop through each line
    for line in input.lines() {
        let result: Vec<&str> = line.split("mul(").collect();

        for (index, item) in result.into_iter().enumerate() {
            // Skip the first item
            if index == 0 {
                continue;
            }

            let comma_split_result = item.split_once(',');

            if comma_split_result.is_none() {
                continue;
            }

            let comma_split = comma_split_result.unwrap();

            let first_multiple_result = comma_split.0.parse::<u32>();

            if first_multiple_result.is_err() {
                continue;
            }

            let paran_split_result = comma_split.1.split_once(')');

            if paran_split_result.is_none() {
                continue;
            }

            let second_multiple_result = paran_split_result.unwrap().0.parse::<u32>();

            if second_multiple_result.is_err() {
                continue;
            }

            total += first_multiple_result.unwrap() * second_multiple_result.unwrap();
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
        let result: Vec<&str> = line.split("mul(").collect();

        for (index, item) in result.into_iter().enumerate() {
            enabled = next_enabled;

            let do_index = item.rfind("do()");
            let dont_index = item.rfind("don't()");

            // If only the do is found
            if do_index.is_some() && dont_index.is_none() {
                next_enabled = true;

            // If only the don't is found
            } else if do_index.is_none() && dont_index.is_some() {
                next_enabled = false;

            // If both are found
            } else if do_index.is_some() && dont_index.is_some() {
                // See which one is the latest
                if do_index.unwrap() > dont_index.unwrap() {
                    next_enabled = true;
                } else {
                    next_enabled = false;
                }
            }

            // Skip the first item
            if index == 0 {
                continue;
            }

            let comma_split_result = item.split_once(',');

            if comma_split_result.is_none() {
                continue;
            }

            let comma_split = comma_split_result.unwrap();

            let first_multiple_result = comma_split_result.unwrap().0.parse::<u32>();

            if first_multiple_result.is_err() {
                continue;
            }

            let paran_split_result = comma_split.1.split_once(')');

            if paran_split_result.is_none() {
                continue;
            }

            let second_multiple_result = paran_split_result.unwrap().0.parse::<u32>();

            if second_multiple_result.is_err() {
                continue;
            }

            if enabled {
                total += first_multiple_result.unwrap() * second_multiple_result.unwrap();
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
