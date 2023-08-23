#[macro_use]
extern crate scan_fmt;
pub fn ParseInput1(Input: &str) -> bool {
    let (start1, end1, start2, end2) =
        scan_fmt!(Input, "{d}-{d},{d}-{d}", u32, u32, u32, u32).unwrap();
    return ((start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1));
}

pub fn ParseInput2(Input: &str) -> bool {
    let (start1, end1, start2, end2) =
        scan_fmt!(Input, "{d}-{d},{d}-{d}", u32, u32, u32, u32).unwrap();
    return ((start1 <= start2 && start2 <= end1) || (start2 <= start1 && start1 <= end2));
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut Result: u32 = 0;
    for line in input.lines() {
        if ParseInput1(line) {
            Result += 1;
        }
    }
    Some(Result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let mut Result: u32 = 0;
    for line in _input.lines() {
        if ParseInput2(line) {
            Result += 1;
        }
    }
    Some(Result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
