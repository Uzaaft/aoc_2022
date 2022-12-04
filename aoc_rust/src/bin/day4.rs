use std::ops::{Range, RangeInclusive};

fn main() {
    let input = include_str!("../../../input/day4.txt");
    let deserialed_contents = deserialize(input);
    dbg!(count_total_overlap(&deserialed_contents));
    dbg!(count_partial_overlap(&deserialed_contents));
}

// Each line contains two ranges, eg. 1-3 or 5-7
// seperated by a comma.
fn deserialize(contents: &str) -> Vec<(u32, u32, u32, u32)> {
    contents
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let first = parts.next().unwrap();
            let second = parts.next().unwrap();
            let mut first_range = first.split('-');
            let mut second_range = second.split('-');

            let first_start = first_range.next().unwrap().parse::<u32>().unwrap();
            let first_end = first_range.next().unwrap().parse::<u32>().unwrap();
            let second_start = second_range.next().unwrap().parse::<u32>().unwrap();
            let second_end = second_range.next().unwrap().parse::<u32>().unwrap();

            (first_start, first_end, second_start, second_end)
        })
        .collect::<Vec<_>>()
}

fn count_partial_overlap(ranges: &Vec<(u32, u32, u32, u32)>) -> u32{
    let mut count = 0;
    for (first_start, first_end, second_start, second_end) in ranges {
        let first_range = first_start..=first_end;
        let second_range = second_start..=second_end;
        if first_range.contains(&second_start) || second_range.contains(&first_start) {
            count += 1;
        }
    }
    count
}

fn count_total_overlap(
    ranges: &Vec<(u32, u32, u32, u32)>,
) -> u32 {
    let mut count = 0;
    for r in ranges {
            let first_interfal = r.0..=r.1;
            let second_interval = r.2..=r.3;
            if first_interfal.contains(&r.2) && first_interfal.contains(&r.3)
                || second_interval.contains(&r.0) && second_interval.contains(&r.1)
            {
                count += 1;
                dbg!(&r.0, &r.1, &r.2, &r.3);
            } else {
                count += 0;
            }
        }
       count
}

#[cfg(test)]
mod tests {
    static contents: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test1() {
        let ranges = super::deserialize(contents);
        let count = super::count_total_overlap(ranges);
        assert_eq!(count, 2);

    }
}
