use std::ops::RangeInclusive;

use itertools::Itertools;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let rows = input.lines().map(str::as_bytes).collect_vec();

    rows.iter()
        .enumerate()
        .map(|(i, row)| {
            let groups = row.iter().enumerate().group_by(|(_, b)| b.is_ascii_digit());

            groups
                .into_iter()
                .filter_map(|(digits, group)| {
                    let group = group.collect_vec();

                    if digits && !group.is_empty() {
                        Some(group.first().unwrap().0..=group.last().unwrap().0)
                    } else {
                        None
                    }
                })
                .filter_map(|range| {
                    for j in range.clone() {
                        if neighbors(i as isize, j as isize, &rows, |cell| {
                            cell != b'.' && !cell.is_ascii_digit()
                        }) > 0
                        {
                            let n = row.get(range).unwrap();
                            let s = unsafe { std::str::from_utf8_unchecked(n) };

                            return Some(s.parse::<u32>().unwrap());
                        }
                    }
                    None
                })
                .sum::<u32>()
        })
        .sum1()
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = input.lines().map(str::as_bytes).collect_vec();

    let stars = rows
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, b)| (*b == b'*').then_some((i, j)))
        })
        .collect_vec();

    let digits = rows
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            let groups = row.iter().enumerate().group_by(|(_, b)| b.is_ascii_digit());

            groups
                .into_iter()
                .filter_map(|(digits, group)| {
                    let group = group.collect_vec();

                    if digits && !group.is_empty() {
                        Some((i, group.first().unwrap().0..=group.last().unwrap().0))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    stars
        .iter()
        .filter_map(|(i, j)| {
            let adjacent = digits
                .iter()
                .filter(|dig| adjacent(*i as isize, *j as isize, dig))
                .collect_vec();
            if adjacent.len() == 2 {
                let fst = unsafe {
                    std::str::from_utf8_unchecked(&rows[adjacent[0].0][adjacent[0].1.clone()])
                };
                let snd = unsafe {
                    std::str::from_utf8_unchecked(&rows[adjacent[1].0][adjacent[1].1.clone()])
                };
                Some(fst.parse::<u32>().unwrap() * snd.parse::<u32>().unwrap())
            } else {
                None
            }
        })
        .sum1()
}

fn neighbors(i: isize, j: isize, rows: &[&[u8]], f: impl Fn(u8) -> bool) -> usize {
    let (w, h) = (rows[0].len() as isize, rows.len() as isize);

    let mut n = 0;

    for (k, l) in [
        (i - 1, j),
        (i + 1, j),
        (i, j - 1),
        (i, j + 1),
        (i - 1, j - 1),
        (i - 1, j + 1),
        (i + 1, j - 1),
        (i + 1, j + 1),
    ] {
        if k >= 0 && k < h && l >= 0 && l < w && f(rows[k as usize][l as usize]) {
            n += 1;
        }
    }

    n
}

fn adjacent(i: isize, j: isize, (row, span): &(usize, RangeInclusive<usize>)) -> bool {
    let neighs = [
        (i - 1, j),
        (i + 1, j),
        (i, j - 1),
        (i, j + 1),
        (i - 1, j - 1),
        (i - 1, j + 1),
        (i + 1, j - 1),
        (i + 1, j + 1),
    ];

    for col in span.clone() {
        if neighs.contains(&(*row as isize, col as isize)) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
