advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list_left = Vec::new();
    let mut list_right = Vec::new();

    // Iterate through the list and grab the left and right values
    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();

        list_left.push(iter.next().unwrap().parse::<u32>().unwrap());
        list_right.push(iter.next().unwrap().parse::<u32>().unwrap());
    }

    // Sort both lists
    list_left.sort();
    list_right.sort();

    let mut sum = 0;

    for (index, item) in list_left.into_iter().enumerate() {
        let difference = u32::abs_diff(item, list_right[index]);
        sum = sum + difference;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list_left = Vec::new();
    let mut list_right = Vec::new();

    // Iterate through the list and grab the left and right values
    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();

        list_left.push(iter.next().unwrap().parse::<u32>().unwrap());
        list_right.push(iter.next().unwrap().parse::<u32>().unwrap());
    }

    // Sort both lists
    list_left.sort();
    list_right.sort();

    let mut sum = 0;

    for left_item in list_left {
        let mut occurences = 0;

        for right_item in &list_right {
            if left_item == *right_item {
                occurences = occurences + 1;
            }
        }

        let similarity_score = left_item * occurences;

        sum = sum + similarity_score;
    }

    Some(sum)
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
