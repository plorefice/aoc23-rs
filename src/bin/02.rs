advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .filter_map(|line| {
            let (game, record) = line.split_once(": ").unwrap();

            let id = game.split_once(' ').unwrap().1.parse::<u32>().unwrap();
            let draws = record.split("; ").map(|r| r.split(", "));

            for draw in draws {
                for cubes in draw {
                    let (n, color) = cubes.split_once(' ').unwrap();
                    let n = n.parse::<u32>().unwrap();

                    match color {
                        "red" if n > 12 => return None,
                        "green" if n > 13 => return None,
                        "blue" if n > 14 => return None,
                        _ => (),
                    }
                }
            }

            Some(id)
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let (_, record) = line.split_once(": ").unwrap();

            let (r, g, b) = record
                .split("; ")
                .fold((0, 0, 0), |(mut r, mut g, mut b), draw| {
                    for cubes in draw.split(", ") {
                        let (n, color) = cubes.split_once(' ').unwrap();
                        let n = n.parse::<u32>().unwrap();

                        match color {
                            "red" => r = r.max(n),
                            "green" => g = g.max(n),
                            "blue" => b = b.max(n),
                            _ => unreachable!(),
                        }
                    }

                    (r, g, b)
                });

            r * g * b
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
