advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(
        input,
        &[
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0",
            "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ],
    )
}

fn solve(input: &str, patterns: &[&str]) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let fst = patterns
                .iter()
                .filter_map(|pat| line.find(pat).map(|i| (i, pat)))
                .min_by_key(|&(i, _)| i)
                .unwrap()
                .1;

            let snd = patterns
                .iter()
                .filter_map(|pat| line.rfind(pat).map(|i| (i, pat)))
                .max_by_key(|&(i, _)| i)
                .unwrap()
                .1;

            digits_to_num(fst) * 10 + digits_to_num(snd)
        })
        .sum::<u32>()
        .into()
}

fn digits_to_num(s: &str) -> u32 {
    match s {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => (s.as_bytes()[0] - b'0') as u32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
